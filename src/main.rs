//pegar o input do usuario
//pegar o input do usuario que diz qual eh a moeda dele
//pegar o input do usuario que deseja para qual moeda seja convertida

//calcular valor da moeda com os dados constantes

//retornar o valor na tela
use std::io;
const DOLLAR_MULTIPLIER: f32 = 5.2;

fn main() {
    println!("Type the amount of money");
    let money_amount = get_user_input_from_terminal();
    let currency_input = get_user_actual_currency_from_terminal();
    let desired_currency = get_user_desired_currency_from_terminal(&currency_input);

    let result: f32 = calculate_conversion(&money_amount, &currency_input, &desired_currency);
    println!("Converted Value: {:.2}", result);
}


fn calculate_conversion(money_amount: &f32, current_currency: &Currencies, desired_currency: &Currencies) -> f32
{
    let value: f32;
    match current_currency {
        Currencies::Real => {
            match desired_currency {
                Currencies::Dollar => value = convert_real_to_dollar(money_amount),
                Currencies::Real => value = *money_amount
            }
        },
        Currencies::Dollar => {
            match desired_currency {
                Currencies::Real => value = convert_dollar_to_real(money_amount),
                Currencies::Dollar => value = *money_amount
            }
        }
    }
    value
}
enum Currencies {
    Dollar,
    Real
}


fn convert_dollar_to_real(amount: &f32) -> f32{
   amount / DOLLAR_MULTIPLIER
}
fn convert_real_to_dollar(amount: &f32) -> f32{
    amount * DOLLAR_MULTIPLIER
}


fn get_user_input_from_terminal() -> f32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error to read the input");
    input.trim().to_string().parse().unwrap()
}

fn get_user_actual_currency_from_terminal()-> Currencies{
    println!("Select your currency:");
    println!("Dollar - 1");
    println!("Real - 2");
    let currency_type = get_currency_type_input();
    currency_type
}

fn get_user_desired_currency_from_terminal(already_chosen_currency: &Currencies) -> Currencies {
    println!("Select your desired currency: ");
    match already_chosen_currency {
        Currencies::Dollar => {
            println!("Real - 2");
        },
        Currencies::Real => {
            println!("Dolar - 1");

        },
    }

    let currency_type = get_currency_type_input();
    currency_type

}

fn get_currency_type_input() -> Currencies{
    let mut currency_inputed = String::new();
    io::stdin().read_line(&mut currency_inputed).expect("Error getting the input");
    
    let currency_id = currency_inputed.trim().parse::<i32>().expect("Error getting the input");
    let mut currency_type = Currencies::Dollar;
    if currency_id > 2 || currency_id.is_negative() {
        currency_type = get_user_actual_currency_from_terminal();
    } else if currency_id == 1 {
        currency_type = Currencies::Dollar;
    } else if currency_id == 2 {
        currency_type = Currencies::Real;
    };
    currency_type
}

