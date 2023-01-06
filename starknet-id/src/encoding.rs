use num_traits::cast::ToPrimitive;
use starknet_core::types::FieldElement;

const BASIC_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789-";
const BIG_ALPHABET: &str = "这来";

#[derive(Debug)]
pub enum EncodingError {
    UnkwnownCharacter(char),
}

fn extract_stars(mut domain: &str) -> (&str, usize) {
    let mut k = 0;
    let last_char = BIG_ALPHABET.chars().last().unwrap();
    while domain.ends_with(last_char) {
        let mut chars = domain.chars();
        chars.next_back();
        domain = chars.as_str();
        k = k + 1;
    }
    (domain, k)
}

pub fn encode(domain: &str) -> Result<FieldElement, EncodingError> {
    let mut mul = 1;
    let mut output = FieldElement::ZERO;
    let mut wip_domain: String;

    if domain.chars().count() >= 2
        && domain.chars().nth(domain.chars().count() - 2).unwrap()
            == BIG_ALPHABET.chars().next().unwrap()
        && domain.chars().last().unwrap() == BASIC_ALPHABET.chars().last().unwrap()
    {
        let mut chars = domain.chars();
        chars.next_back();
        chars.next_back();
        let (str, k) = extract_stars(chars.as_str());
        wip_domain = String::from(str);
        wip_domain.push_str(
            BIG_ALPHABET
                .chars()
                .last()
                .unwrap()
                .to_string()
                .repeat(2 * (k + 1))
                .as_str(),
        )
    } else {
        let (str, k) = extract_stars(domain);
        if k != 0 {
            wip_domain = String::from(str);
            wip_domain.push_str(
                BIG_ALPHABET
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
                    .repeat(1 + (2 * (k - 1)))
                    .as_str(),
            )
        } else {
            wip_domain = String::from(domain);
        }
    }

    for (i, c) in wip_domain.chars().enumerate() {
        if i == wip_domain.chars().count() - 1 && c == BASIC_ALPHABET.chars().next().unwrap() {
            output = output + FieldElement::from(BASIC_ALPHABET.chars().count() * mul);
        } else {
            let found_basic = BASIC_ALPHABET
                .chars()
                .position(|alphabet_c| alphabet_c == c);

            match found_basic {
                Some(index) => {
                    output = output + FieldElement::from(index * mul);
                    mul = mul * (BASIC_ALPHABET.chars().count() + 1);
                }
                None => {
                    let found_big = BIG_ALPHABET.chars().position(|alphabet_c| alphabet_c == c);
                    match found_big {
                        Some(index) => {
                            output =
                                output + FieldElement::from(BASIC_ALPHABET.chars().count() * mul);
                            mul = mul * (BASIC_ALPHABET.chars().count() + 1);

                            output = output
                                + FieldElement::from(
                                    mul * (index
                                        + if i == wip_domain.chars().count() - 1 {
                                            1
                                        } else {
                                            0
                                        }),
                                );
                            mul = mul * BIG_ALPHABET.chars().count();
                        }
                        None => {
                            return Err(EncodingError::UnkwnownCharacter(c));
                        }
                    }
                }
            }
        }
    }

    return Ok(output);
}

pub fn decode(mut felt: FieldElement) -> String {
    let mut decoded: String = String::new();
    let basic_plus = FieldElement::from(BASIC_ALPHABET.chars().count() + 1);
    let basic_len = FieldElement::from(BASIC_ALPHABET.chars().count());
    let big_plus = FieldElement::from(BIG_ALPHABET.chars().count() + 1);
    let big_len = FieldElement::from(BIG_ALPHABET.chars().count());
    let last_big = BIG_ALPHABET.chars().last().unwrap();
    while felt != FieldElement::ZERO {
        let code = felt % basic_plus;
        felt = felt.floor_div(basic_plus);
        if code == basic_len {
            let next_felt = felt.floor_div(big_plus);
            if next_felt == FieldElement::ZERO {
                let code2 = felt % big_plus;
                felt = next_felt;
                decoded.push(if code2 == FieldElement::ZERO {
                    BASIC_ALPHABET.chars().next().unwrap()
                } else {
                    last_big
                });
            } else {
                decoded.push(
                    BIG_ALPHABET
                        .chars()
                        .nth((felt % big_len).to_big_decimal(0).to_usize().unwrap())
                        .unwrap(),
                );
                felt = felt.floor_div(big_len);
            }
        } else {
            decoded.push(
                BASIC_ALPHABET
                    .chars()
                    .nth(code.to_big_decimal(0).to_usize().unwrap())
                    .unwrap(),
            );
        }

        let (decoded_str, k) = extract_stars(decoded.as_str());
        let mut decoded = String::from(decoded_str);
        if k != 0 {
            let star = last_big.to_string();
            if k % 2 == 0 {
                decoded.push_str(&str::repeat(&star, k / 2 - 1));
                decoded.push(BIG_ALPHABET.chars().next().unwrap());
                let mut basic_iter = BASIC_ALPHABET.chars();
                basic_iter.next();
                decoded.push(basic_iter.next().unwrap());
            } else {
                decoded.push_str(&str::repeat(&star, k / 2 + 1));
            }
        }
    }
    decoded
}
