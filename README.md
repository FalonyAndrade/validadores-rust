Como instalar { Cargo.toml }
``` Crate
[dependencies]
validadores = "1.0.0"
```

Como utilizar
```rust
use validadores as vd;
use std::io;

fn main() {
    println!("Digite um CPF");

    let mut cpf = String::new();

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("Voce digitou {}", cpf.trim());
        },
        Err(e) => {
            println!("Erro ao ler entrada: {}", e);
        }
    }


    let validado = vd::validador::cpf(cpf.as_str());
    //let validado = vd::validador::cnpj(cnpj.as_str()); // ou CNPJ
    //let validado = vd::validador::rg(rg.as_str()); // ou RG

    if validado {
        println!("O CPF é valido");
    } else {
        println!("O CPF é invalido")
    }
}
