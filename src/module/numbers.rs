// Functions for numbers

pub const fn sqrt_newton(number: f64) -> f64 {
    if number < 0.0 {
        panic!("Cannot compute the square root of a negative number");
    }
    if number == 0.0 {
        return 0.0;
    }

    let mut guess = number / 2.0;
    let mut prev_guess;

    loop {
        prev_guess = guess;
        guess = (guess + number / guess) / 2.0;
        if absolute(prev_guess - guess) < 1e-10 {
            break;
        }
    }

    guess
}

pub const fn absolute(value: f64) -> f64 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}