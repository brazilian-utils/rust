# Brazilian Utils - Rust

[![CI](https://github.com/YOUR_USERNAME/brazilian-utils-rust/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/brazilian-utils-rust/actions)
[![Crates.io](https://img.shields.io/crates/v/brazilian_utils.svg)](https://crates.io/crates/brazilian_utils)
[![Documentation](https://docs.rs/brazilian_utils/badge.svg)](https://docs.rs/brazilian_utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](#english) | [Português](#português)

---

## English

A Rust library providing utility functions for Brazilian-specific data validation, formatting, and generation.

### Features

This library provides comprehensive utilities for handling Brazilian documents, identifiers, and data formats:

#### 📋 Document Validation & Formatting

- **CPF** (Cadastro de Pessoas Físicas) - Individual Taxpayer Registry
- **CNPJ** (Cadastro Nacional da Pessoa Jurídica) - National Registry of Legal Entities
- **CNH** (Carteira Nacional de Habilitação) - National Driver's License
- **PIS** (Programa de Integração Social) - Social Integration Program
- **Voter ID** (Título de Eleitor) - Electoral Registration

#### 🚗 Vehicle & Transportation

- **License Plate** - Old and Mercosul format validation and conversion
- **RENAVAM** (Registro Nacional de Veículos Automotores) - National Motor Vehicle Registry

#### 🏛️ Legal & Administrative

- **Legal Process** - Brazilian legal system process numbers
- **Legal Nature** - Legal entity classification (60+ official codes)

#### 📍 Location & Communication

- **CEP** (Código de Endereçamento Postal) - Postal Code with address lookup
- **Phone** - Mobile and landline validation with formatting

#### 💰 Financial & Text

- **Currency** - Real (BRL) formatting and text conversion
- **Date Utils** - Holiday checking and date text conversion
- **Email** - RFC 5322 compliant validation

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
brazilian_utils = "0.1.0"
```

### Usage Examples

#### CPF Validation and Formatting

```rust
use brazilian_utils::cpf;

// Validate
assert!(cpf::is_valid("11144477735"));

// Format
let formatted = cpf::format_cpf("11144477735");
assert_eq!(formatted, Some("111.444.777-35".to_string()));

// Generate random valid CPF
let cpf_number = cpf::generate();
assert!(cpf::is_valid(&cpf_number));
```

#### CNPJ Validation and Formatting

```rust
use brazilian_utils::cnpj;

// Validate
assert!(cnpj::is_valid("11222333000181"));

// Format
let formatted = cnpj::format_cnpj("11222333000181");
assert_eq!(formatted, Some("11.222.333/0001-81".to_string()));

// Generate random valid CNPJ
let cnpj_number = cnpj::generate(None);
assert!(cnpj::is_valid(&cnpj_number));
```

#### CEP (Postal Code) with Address Lookup

```rust
use brazilian_utils::cep;

// Validate
assert!(cep::is_valid("01310200"));

// Format
let formatted = cep::format_cep("01310200");
assert_eq!(formatted, Some("01310-200".to_string()));

// Get address from CEP (requires internet)
if let Some(address) = cep::get_address_from_cep("01310200") {
    println!("Street: {}", address.street);
    println!("City: {}", address.city);
}
```

#### License Plate (Old and Mercosul)

```rust
use brazilian_utils::license_plate;

// Validate old format
assert!(license_plate::is_valid("ABC1234", Some("old")));

// Validate Mercosul format
assert!(license_plate::is_valid("ABC1D23", Some("mercosul")));

// Convert old to Mercosul
let mercosul = license_plate::convert_to_mercosul("ABC1234");
assert_eq!(mercosul, Some("ABC1C34".to_string()));

// Format
let formatted = license_plate::format_license_plate("ABC1234");
assert_eq!(formatted, Some("ABC-1234".to_string()));
```

#### Currency Formatting and Text Conversion

```rust
use brazilian_utils::currency;

// Format as currency
let formatted = currency::format_currency(1234.56);
assert_eq!(formatted, "R$ 1.234,56");

// Convert to text
let text = currency::convert_real_to_text(1234.56);
assert_eq!(text, "mil duzentos e trinta e quatro reais e cinquenta e seis centavos");
```

#### Phone Number Validation

```rust
use brazilian_utils::phone;

// Validate mobile
assert!(phone::is_valid("11987654321", Some("mobile")));

// Validate landline
assert!(phone::is_valid("1133334444", Some("landline")));

// Format
let formatted = phone::format_phone("11987654321");
assert_eq!(formatted, Some("(11) 98765-4321".to_string()));

// Generate random
let phone_number = phone::generate(Some("mobile"));
assert!(phone::is_valid(&phone_number, Some("mobile")));
```

#### Voter ID (Título de Eleitor)

```rust
use brazilian_utils::voter_id;

// Validate
assert!(voter_id::is_valid("690847092828"));

// Format
let formatted = voter_id::format_voter_id("690847092828");
assert_eq!(formatted, Some("6908 4709 28 28".to_string()));

// Generate for a specific state
let voter_id_sp = voter_id::generate(Some("SP")).unwrap();
assert!(voter_id::is_valid(&voter_id_sp));
```

#### Legal Process Number

```rust
use brazilian_utils::legal_process;

// Validate
assert!(legal_process::is_valid("00000000020248140141"));

// Format
let formatted = legal_process::format_legal_process("00000000020248140141");
assert_eq!(formatted, Some("0000000-00.2024.8.14.0141".to_string()));

// Generate random
let process = legal_process::generate(Some(("8", "14", "0141")));
assert!(legal_process::is_valid(&process.unwrap()));
```

#### Date Utilities

```rust
use brazilian_utils::date_utils;
use chrono::NaiveDate;

// Check if date is a holiday
let christmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
assert!(date_utils::is_holiday(&christmas, None));

// Convert date to text
let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
let text = date_utils::convert_date_to_text(&date);
assert_eq!(text, Some("15 de janeiro de 2024".to_string()));
```

### All Available Modules

| Module | Functions | Description |
|--------|-----------|-------------|
| `cep` | `is_valid`, `format_cep`, `remove_symbols`, `generate`, `get_address_from_cep`, `get_cep_information_from_address` | Postal code validation and address lookup |
| `cnh` | `is_valid_cnh` | Driver's license validation |
| `cnpj` | `is_valid`, `validate`, `format_cnpj`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Company registration validation |
| `cpf` | `is_valid`, `validate`, `format_cpf`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Individual taxpayer validation |
| `currency` | `format_currency`, `convert_real_to_text`, `number_to_words` | Currency formatting and text conversion |
| `date_utils` | `is_holiday`, `convert_date_to_text` | Date utilities and holiday checking |
| `email` | `is_valid` | RFC 5322 email validation |
| `legal_nature` | `is_valid`, `get_description`, `list_all` | Legal entity nature codes (60+ codes) |
| `legal_process` | `is_valid`, `format_legal_process`, `remove_symbols`, `generate` | Legal process number validation |
| `license_plate` | `is_valid`, `format_license_plate`, `remove_symbols`, `convert_to_mercosul`, `get_format`, `generate` | Vehicle license plate (old/Mercosul) |
| `phone` | `is_valid`, `format_phone`, `remove_symbols`, `generate`, `remove_international_dialing_code` | Phone number validation |
| `pis` | `is_valid`, `format_pis`, `remove_symbols`, `generate`, `checksum` | Social integration number |
| `renavam` | `is_valid`, `generate`, `calculate_checksum` | Vehicle registration number |
| `voter_id` | `is_valid`, `format_voter_id`, `generate`, `calculate_vd1`, `calculate_vd2` | Electoral registration validation |

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test cpf

# Run with output
cargo test -- --nocapture
```

### Running Examples

The library includes comprehensive demo examples for each module:

```bash
# CPF demonstration
cargo run --example cpf_demo

# CNPJ demonstration
cargo run --example cnpj_demo

# License Plate demonstration
cargo run --example license_plate_demo

# Voter ID demonstration
cargo run --example voter_id_demo

# And many more...
```

### Test Coverage

- **149 unit tests** covering all validation logic
- **55 documentation tests** ensuring examples work correctly
- **Total: 204 tests** with 100% passing rate

### Dependencies

- `rand` - Random number generation
- `reqwest` - HTTP client for CEP address lookup
- `serde` / `serde_json` - JSON serialization
- `chrono` - Date and time handling
- `regex` - Regular expression matching
- `unicode-normalization` - String normalization

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the MIT License.

---

## Português

Uma biblioteca Rust que fornece funções utilitárias para validação, formatação e geração de dados específicos do Brasil.

### Funcionalidades

Esta biblioteca fornece utilitários abrangentes para manipular documentos, identificadores e formatos de dados brasileiros:

#### 📋 Validação e Formatação de Documentos

- **CPF** (Cadastro de Pessoas Físicas) - Registro de Contribuinte Individual
- **CNPJ** (Cadastro Nacional da Pessoa Jurídica) - Registro Nacional de Pessoas Jurídicas
- **CNH** (Carteira Nacional de Habilitação) - Carteira de Motorista Nacional
- **PIS** (Programa de Integração Social) - Programa de Integração Social
- **Título de Eleitor** - Registro Eleitoral

#### 🚗 Veículos e Transporte

- **Placas de Veículo** - Validação e conversão de formatos antigo e Mercosul
- **RENAVAM** (Registro Nacional de Veículos Automotores) - Registro Nacional de Veículos

#### 🏛️ Legal e Administrativo

- **Processo Judicial** - Números de processo do sistema jurídico brasileiro
- **Natureza Jurídica** - Classificação de entidades jurídicas (60+ códigos oficiais)

#### 📍 Localização e Comunicação

- **CEP** (Código de Endereçamento Postal) - Código Postal com busca de endereço
- **Telefone** - Validação de celular e fixo com formatação

#### 💰 Financeiro e Texto

- **Moeda** - Formatação de Real (BRL) e conversão para texto
- **Utilitários de Data** - Verificação de feriados e conversão de data para texto
- **Email** - Validação compatível com RFC 5322

### Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
brazilian_utils = "0.1.0"
```

### Exemplos de Uso

#### Validação e Formatação de CPF

```rust
use brazilian_utils::cpf;

// Validar
assert!(cpf::is_valid("11144477735"));

// Formatar
let formatado = cpf::format_cpf("11144477735");
assert_eq!(formatado, Some("111.444.777-35".to_string()));

// Gerar CPF válido aleatório
let numero_cpf = cpf::generate();
assert!(cpf::is_valid(&numero_cpf));
```

#### Validação e Formatação de CNPJ

```rust
use brazilian_utils::cnpj;

// Validar
assert!(cnpj::is_valid("11222333000181"));

// Formatar
let formatado = cnpj::format_cnpj("11222333000181");
assert_eq!(formatado, Some("11.222.333/0001-81".to_string()));

// Gerar CNPJ válido aleatório
let numero_cnpj = cnpj::generate(None);
assert!(cnpj::is_valid(&numero_cnpj));
```

#### CEP (Código Postal) com Busca de Endereço

```rust
use brazilian_utils::cep;

// Validar
assert!(cep::is_valid("01310200"));

// Formatar
let formatado = cep::format_cep("01310200");
assert_eq!(formatado, Some("01310-200".to_string()));

// Buscar endereço pelo CEP (requer internet)
if let Some(endereco) = cep::get_address_from_cep("01310200") {
    println!("Rua: {}", endereco.street);
    println!("Cidade: {}", endereco.city);
}
```

#### Placa de Veículo (Antiga e Mercosul)

```rust
use brazilian_utils::license_plate;

// Validar formato antigo
assert!(license_plate::is_valid("ABC1234", Some("old")));

// Validar formato Mercosul
assert!(license_plate::is_valid("ABC1D23", Some("mercosul")));

// Converter antiga para Mercosul
let mercosul = license_plate::convert_to_mercosul("ABC1234");
assert_eq!(mercosul, Some("ABC1C34".to_string()));

// Formatar
let formatado = license_plate::format_license_plate("ABC1234");
assert_eq!(formatado, Some("ABC-1234".to_string()));
```

#### Formatação e Conversão de Moeda

```rust
use brazilian_utils::currency;

// Formatar como moeda
let formatado = currency::format_currency(1234.56);
assert_eq!(formatado, "R$ 1.234,56");

// Converter para texto
let texto = currency::convert_real_to_text(1234.56);
assert_eq!(texto, "mil duzentos e trinta e quatro reais e cinquenta e seis centavos");
```

#### Validação de Número de Telefone

```rust
use brazilian_utils::phone;

// Validar celular
assert!(phone::is_valid("11987654321", Some("mobile")));

// Validar fixo
assert!(phone::is_valid("1133334444", Some("landline")));

// Formatar
let formatado = phone::format_phone("11987654321");
assert_eq!(formatado, Some("(11) 98765-4321".to_string()));

// Gerar aleatório
let numero_telefone = phone::generate(Some("mobile"));
assert!(phone::is_valid(&numero_telefone, Some("mobile")));
```

#### Título de Eleitor

```rust
use brazilian_utils::voter_id;

// Validar
assert!(voter_id::is_valid("690847092828"));

// Formatar
let formatado = voter_id::format_voter_id("690847092828");
assert_eq!(formatado, Some("6908 4709 28 28".to_string()));

// Gerar para um estado específico
let titulo_sp = voter_id::generate(Some("SP")).unwrap();
assert!(voter_id::is_valid(&titulo_sp));
```

#### Número de Processo Judicial

```rust
use brazilian_utils::legal_process;

// Validar
assert!(legal_process::is_valid("00000000020248140141"));

// Formatar
let formatado = legal_process::format_legal_process("00000000020248140141");
assert_eq!(formatado, Some("0000000-00.2024.8.14.0141".to_string()));

// Gerar aleatório
let processo = legal_process::generate(Some(("8", "14", "0141")));
assert!(legal_process::is_valid(&processo.unwrap()));
```

#### Utilitários de Data

```rust
use brazilian_utils::date_utils;
use chrono::NaiveDate;

// Verificar se é feriado
let natal = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
assert!(date_utils::is_holiday(&natal, None));

// Converter data para texto
let data = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
let texto = date_utils::convert_date_to_text(&data);
assert_eq!(texto, Some("15 de janeiro de 2024".to_string()));
```

### Todos os Módulos Disponíveis

| Módulo | Funções | Descrição |
|--------|---------|-----------|
| `cep` | `is_valid`, `format_cep`, `remove_symbols`, `generate`, `get_address_from_cep`, `get_cep_information_from_address` | Validação de CEP e busca de endereço |
| `cnh` | `is_valid_cnh` | Validação de CNH |
| `cnpj` | `is_valid`, `validate`, `format_cnpj`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Validação de registro empresarial |
| `cpf` | `is_valid`, `validate`, `format_cpf`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Validação de CPF |
| `currency` | `format_currency`, `convert_real_to_text`, `number_to_words` | Formatação e conversão de moeda |
| `date_utils` | `is_holiday`, `convert_date_to_text` | Utilitários de data e verificação de feriados |
| `email` | `is_valid` | Validação de email RFC 5322 |
| `legal_nature` | `is_valid`, `get_description`, `list_all` | Códigos de natureza jurídica (60+ códigos) |
| `legal_process` | `is_valid`, `format_legal_process`, `remove_symbols`, `generate` | Validação de número de processo |
| `license_plate` | `is_valid`, `format_license_plate`, `remove_symbols`, `convert_to_mercosul`, `get_format`, `generate` | Placa de veículo (antiga/Mercosul) |
| `phone` | `is_valid`, `format_phone`, `remove_symbols`, `generate`, `remove_international_dialing_code` | Validação de telefone |
| `pis` | `is_valid`, `format_pis`, `remove_symbols`, `generate`, `checksum` | Número de integração social |
| `renavam` | `is_valid`, `generate`, `calculate_checksum` | Número de registro de veículo |
| `voter_id` | `is_valid`, `format_voter_id`, `generate`, `calculate_vd1`, `calculate_vd2` | Validação de título de eleitor |

### Executando Testes

```bash
# Executar todos os testes
cargo test

# Executar testes de um módulo específico
cargo test cpf

# Executar com saída detalhada
cargo test -- --nocapture
```

### Executando Exemplos

A biblioteca inclui exemplos de demonstração abrangentes para cada módulo:

```bash
# Demonstração de CPF
cargo run --example cpf_demo

# Demonstração de CNPJ
cargo run --example cnpj_demo

# Demonstração de Placa de Veículo
cargo run --example license_plate_demo

# Demonstração de Título de Eleitor
cargo run --example voter_id_demo

# E muitos mais...
```

### Cobertura de Testes

- **149 testes unitários** cobrindo toda a lógica de validação
- **55 testes de documentação** garantindo que os exemplos funcionem corretamente
- **Total: 204 testes** com 100% de aprovação

### Dependências

- `rand` - Geração de números aleatórios
- `reqwest` - Cliente HTTP para busca de endereço por CEP
- `serde` / `serde_json` - Serialização JSON
- `chrono` - Manipulação de data e hora
- `regex` - Correspondência de expressões regulares
- `unicode-normalization` - Normalização de strings

### Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para enviar um Pull Request.

### Licença

Este projeto está licenciado sob a Licença MIT.

---

## Acknowledgments / Agradecimentos

Inspired by [brazilian-utils/python](https://github.com/brazilian-utils/python) - A Python library with similar utilities for Brazilian data.

Inspirado em [brazilian-utils/python](https://github.com/brazilian-utils/python) - Uma biblioteca Python com utilitários similares para dados brasileiros.
