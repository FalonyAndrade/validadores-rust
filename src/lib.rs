pub fn validar_cpf(cpf: &str) -> bool {
    let cpf: Vec<u8> = cpf
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    if cpf.len() != 11 || cpf.iter().all(|&d| d == cpf[0]) {
        return false;
    }

    let mut soma = 0;
    for i in 0..9 {
        soma += (cpf[i] as usize) * (10 - i);
    }

    let resto = (soma * 10) % 11;
    let primeiro_digito = if resto == 10 { 0 } else { resto as u8 };

    soma = 0;
    for i in 0..10 {
        soma += (cpf[i] as usize) * (11 - i);
    }
    let resto = (soma * 10) % 11;
    let segundo_digito = if resto == 10 { 0 } else { resto as u8 };

    primeiro_digito == cpf[9] && segundo_digito == cpf[10]
}

