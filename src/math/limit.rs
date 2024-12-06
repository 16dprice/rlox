struct Term {
    coefficient: f64,
    power: i32,
}

struct Polynomial {
    terms: Vec<Term>,
}

impl Polynomial {
    pub fn compute(&self, x: f64) -> f64 {
        let mut total = 0.0;
        for term in &self.terms {
            total += term.coefficient * x.powi(term.power);
        }
        return total;
    }
}

struct PolynomialParser {
    expression: String,
}

impl PolynomialParser {
    fn parse_polynomial(&self) -> Result<Polynomial, String> {
        let mut terms = Vec::new();

        for term in self.expression.split_whitespace() {
            println!("{}", term);
        }

        return Ok(Polynomial { terms });
    }
}

// TODO: Consider renaming this
pub fn parse_polynomial(expression: String) {
    let parser = PolynomialParser { expression };
    let polynomial = parser.parse_polynomial();

    println!("{}", polynomial.unwrap().compute(1.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wip() {
        parse_polynomial(String::from("x^2 + x - 1"));
    }
}
