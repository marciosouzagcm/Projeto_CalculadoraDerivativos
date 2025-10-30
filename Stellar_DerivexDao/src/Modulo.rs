use std::error::Error;

/// Consulta o saldo de tokens na pool.
/// # Retorno
/// * `Result<f64, Box<dyn Error>>`: Saldo de tokens na pool.
pub fn get_token_reserves(&self) -> Result<f64, Box<dyn Error>> {
    // Criação do ativo.
    let asset = Asset::new(&self.token_address);
    
    // Carregamento do saldo da conta.
    let balance = self.server.load_account(&self.factory_address)?
        .balance(&asset)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    Ok(balance)
}

/// Calcula quantos tokens o utilizador recebe ao vender XLM.
/// # Parâmetros
/// * `xlm_sold`: Quantidade de XLM vendida.
/// # Retorno
/// * `f64`: Quantidade de tokens recebida.
pub fn get_token_amount(&self, xlm_sold: f64) -> Result<f64, Box<dyn Error>> {
    if xlm_sold <= 0.0 {
        return Err("A quantidade de XLM vendida deve ser maior que zero.".into());
    }
    
    // Consulta do saldo de tokens na pool.
    let output_reserve = self.get_token_reserves()?;
    
    // Cálculo da quantidade de tokens recebida.
    let input_amount_with_fee = xlm_sold * 0.997;
    let numerator = input_amount_with_fee * output_reserve;
    let denominator = (output_reserve * 1000.0) + input_amount_with_fee;
    
    Ok(numerator / denominator)
}

/// Calcula quanto XLM o utilizador recebe ao vender tokens.
/// # Parâmetros
/// * `tokens_sold`: Quantidade de tokens vendida.
/// # Retorno
/// * `f64`: Quantidade de XLM recebida.
pub fn get_xlm_amount(&self, tokens_sold: f64) -> Result<f64, Box<dyn Error>> {
    if tokens_sold <= 0.0 {
        return Err("A quantidade de tokens vendida deve ser maior que zero.".into());
    }
    
    // Consulta do saldo de tokens na pool.
    let input_reserve = self.get_token_reserves()?;
    
    // Cálculo da quantidade de XLM recebida.
    let output_amount = tokens_sold * 0.997;
    let numerator = output_amount * input_reserve;
    let denominator = (input_reserve * 1000.0) + output_amount;
    
    Ok(numerator / denominator)
}

/// Permite que os utilizadores adicionem liquidez à pool.
/// # Parâmetros
/// * `tokens_added`: Quantidade de tokens adicionada.
/// # Retorno
/// * `f64`: Quantidade de XLM adicionada.
pub fn add_liquidity(&self, tokens_added: f64) -> Result<f64, Box<dyn Error>> {
    if tokens_added <= 0.0 {
        return Err("A quantidade de tokens adicionada deve ser maior que zero.".into());
    }
    
    // Carregamento do saldo da conta.
    let xlm_balance = self.server.load_account(&self.factory_address)?
        .balance;
    
    // Consulta do saldo de tokens na pool.
    let token_balance = self.get_token_reserves()?;
    
    // Cálculo da quantidade de XLM adicionada.
    if token_balance == 0.0 {
        Ok(xlm_balance)
    } else {
        Ok((xlm_balance * tokens_added) / token_balance)
    }
}

/// Permite que os utilizadores removam liquidez da pool.
/// # Parâmetros
/// * `liquidity_removed`: Quantidade de liquidez removida.
/// # Retorno
/// * `(f64, f64)`: Quantidade de XLM e tokens removidos.
pub fn remove_liquidity(&self, liquidity_removed: f64) -> Result<(f64, f64), Box<dyn Error>> {
    if liquidity_removed <= 0.0 {
        return Err("A quantidade de liquidez removida deve ser maior que zero.".into());
    }
    
    // Carregamento do saldo da conta.
    let xlm_balance = self.server.load_account(&self.factory_address)?
        .balance;
    
    // Consulta do saldo de tokens na pool.
    let token_balance = self.get_token_reserves()?;
    
    // Calculando a liquidez de XLM e tokens removidos
    let xlm_removed = (xlm_balance * liquidity_removed) / token_balance;
    let tokens_removed = liquidity_removed; // Implementar lógica real de remoção de tokens
    
    Ok((xlm_removed, tokens_removed))
}
