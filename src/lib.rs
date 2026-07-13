pub mod validador_doc {
    pub fn cpf(cpf: &str) -> bool {
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

    pub fn cnpj(cnpj_texto: &str) -> bool {
        let vec: Vec<u8> = cnpj_texto
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect();

        if vec.len() != 14 || vec.iter().all(|&d| d == vec[0]) {
            return false;
        }

        let digito_validado = |tamanho_analisado: usize| -> bool {
            let mut soma = 0;
            
            let mut peso = tamanho_analisado - 7;

            for &num in vec.iter().take(tamanho_analisado) {
                soma += num as usize * peso;
                peso = if peso == 2 { 9 } else { peso - 1 };
            }

            let resto = soma % 11;
            let digito_calculado = if resto < 2 { 0 } else { 11 - resto };

            digito_calculado == vec[tamanho_analisado] as usize
        };

        digito_validado(12) && digito_validado(13)

    }

    pub fn rg(rg_texto: &str) -> bool {
        let mut digitos: Vec<char> = rg_texto
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == 'X' || *c == 'x')
            .collect();

        if digitos.len() != 9 {
            return false;
        }

        let digito_verificador_real = match digitos.pop() {
            Some('X') | Some('x') => 10,
            Some(c) => c.to_digit(10).unwrap() as usize,
            None => return false,
        };

        let mut numeros = Vec::with_capacity(8);
        for c in digitos {
            if let Some(num) = c.to_digit(10) {
                numeros.push(num as usize);
            } else {
                return false;
            }
        }

        let mut soma = 0;
        for i in 0..8 {
            let peso = i + 2; 
            soma += numeros[i] * peso;
        }

        let resto = soma % 11;

        let digito_esperado = match resto {
            0 => 0,
            1 => 10, // Representa o 'X'
            _ => 11 - resto,
        };

        digito_verificador_real == digito_esperado

    }

}