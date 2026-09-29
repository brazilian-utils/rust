# Brazilian Utils - Rust

[![CI](https://github.com/brazilian-utils/rust/workflows/CI/badge.svg)](https://github.com/brazilian-utils/rust/actions)
[![Crates.io](https://img.shields.io/crates/v/brazilian_utils.svg)](https://crates.io/crates/brazilian_utils)
[![Documentation](https://docs.rs/brazilian_utils/badge.svg)](https://docs.rs/brazilian_utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Português](#português) | [English](#english)

---

## Português

Uma biblioteca Rust que fornece funções utilitárias para validação, formatação e geração de dados específicos do Brasil.

### Funcionalidades

Esta biblioteca fornece utilitários abrangentes para manipular documentos, identificadores e formatos de dados brasileiros:

#### 📋 Validação e Formatação de Documentos

- **CPF** (Cadastro de Pessoas Físicas) - Registro de Contribuinte Individual
- **CNPJ** (Cadastro Nacional da Pessoa Jurídica) - Registro Nacional de Pessoas Jurídicas, incluindo o novo formato alfanumérico (IN RFB nº 2.119/2021)
- **CNH** (Carteira Nacional de Habilitação) - Carteira de Motorista Nacional
- **PIS** (Programa de Integração Social) - Programa de Integração Social
- **Título de Eleitor** - Registro Eleitoral

#### 🆔 Documentos e Cadastros Adicionais

- **Passaporte** - Validação e geração de número de passaporte brasileiro
- **CNS** (Cartão Nacional de Saúde) - Validação e formatação do cartão do SUS
- **Certidão de Registro Civil** - Validação e leitura da matrícula (nascimento, casamento, óbito etc., art. 473 do CNNCNJ)
- **CAEPF** (Cadastro de Atividade Econômica da Pessoa Física) - Validação e formatação
- **CEI** (Cadastro Específico do INSS) - Validação e formatação
- **CNO** (Cadastro Nacional de Obras) - Validação e formatação

#### 🚗 Veículos e Transporte

- **Placas de Veículo** - Validação e conversão de formatos antigo e Mercosul
- **RENAVAM** (Registro Nacional de Veículos Automotores) - Registro Nacional de Veículos
- **VIN** (Chassi do Veículo) - Validação estrutural com dígito verificador norte-americano

#### 🏛️ Legal e Administrativo

- **Processo Judicial** - Números de processo do sistema jurídico brasileiro
- **Natureza Jurídica** - Classificação de entidades jurídicas (92 códigos oficiais, Natureza Jurídica 2021)
- **Registro Profissional** - Validação estrutural de OAB, CRM, CRO, CRP e CRC

#### 📊 Fiscal e Classificações Oficiais

- **CFOP** (Código Fiscal de Operações e Prestações) - Validação e consulta de descrição oficial
- **CNAE** (Classificação Nacional de Atividades Econômicas) - Validação, formatação e consulta
- **CSOSN** (Código de Situação da Operação no Simples Nacional) - Validação dos 10 códigos oficiais
- **CST** (Código de Situação Tributária) - Validação de ICMS, IPI, PIS e COFINS
- **NCM** (Nomenclatura Comum do Mercosul) - Validação e formatação de códigos aduaneiros
- **Chave de Acesso da NF-e** - Validação e leitura da chave de 44 dígitos (NF-e, NFC-e, CT-e, MDF-e e outros DF-e)
- **Inscrição Estadual (IE)** - Validação estrutural por estado
- **CBO** (Classificação Brasileira de Ocupações) - Validação e consulta de descrição oficial

#### 💳 Bancário e Pagamentos

- **Banco** - Consulta por código COMPE ou ISPB
- **Conta Bancária** - Validação estrutural de agência, conta e dígito verificador
- **IBAN** - Validação, formatação e leitura do IBAN brasileiro
- **Chave Pix** - Identificação e validação de chaves CPF, CNPJ, email, telefone e EVP
- **Payload Pix** (BR Code) - Validação, leitura e geração de "Pix copia e cola"
- **Cartão de Crédito/Débito** - Validação pelo algoritmo de Luhn

#### 📍 Localização e Comunicação

- **CEP** (Código de Endereçamento Postal) - Código Postal com busca de endereço
- **Telefone** - Validação de celular e fixo com formatação
- **DDD** (Código de Área) - Consulta de estado e região por código de área
- **Município** - Consulta por código IBGE e listagem por estado
- **Estado (UF)** - Consulta de nome, código, fuso horário e listagem

#### 💰 Financeiro e Texto

- **Boleto** - Validação de linha digitável de boleto bancário
- **Moeda** - Formatação de Real (BRL) e conversão para texto
- **Utilitários de Data** - Verificação de feriados (fixos, móveis e estaduais), dias úteis e conversão de data para texto
- **Email** - Validação compatível com RFC 5322
- **Número por Extenso** - Conversão de números em texto, com concordância de gênero
- **Texto** - Capitalização de nomes/empresas e remoção de acentos

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
assert!(cnpj::is_valid("03560714000142", None));

// Formatar
let formatado = cnpj::format_cnpj("03560714000142");
assert_eq!(formatado, Some("03.560.714/0001-42".to_string()));

// Gerar CNPJ válido aleatório
let numero_cnpj = cnpj::generate(None, None);
assert!(cnpj::is_valid(&numero_cnpj, None));

// Novo formato alfanumérico (v2, IN RFB nº 2.119/2021)
let cnpj_v2 = cnpj::generate(None, Some(2));
assert!(cnpj::is_valid(&cnpj_v2, Some(2)));
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

#### Validação de Boleto

```rust
use brazilian_utils::boleto;

// Validar linha digitável de boleto
assert!(boleto::is_valid("00190000090114971860168524522114675860000102656"));

// Validar com formatação (espaços e pontos)
assert!(boleto::is_valid("0019000009 01149.718601 68524.522114 6 75860000102656"));

// Validar linha inválida
assert!(!boleto::is_valid("00190000020114971860168524522114675860000102656"));
```

#### Placa de Veículo (Antiga e Mercosul)

```rust
use brazilian_utils::license_plate;

// Validar (aceita formato antigo ou Mercosul)
assert!(license_plate::is_valid("ABC1234"));
assert!(license_plate::is_valid("ABC1D23"));

// Validar um formato específico
assert!(license_plate::is_valid_for_format("ABC1234", Some("old_format")));
assert!(license_plate::is_valid_for_format("ABC1D23", Some("mercosul")));

// Converter antiga para Mercosul
let mercosul = license_plate::convert_to_mercosul("ABC1234");
assert_eq!(mercosul, "ABC1C34".to_string());

// Formatar
let formatado = license_plate::format_license_plate("ABC1234");
assert_eq!(formatado, Some("ABC-1234".to_string()));
```

#### VIN (Chassi do Veículo)

```rust
use brazilian_utils::vin;

// Validar (dígito verificador norte-americano)
assert!(vin::is_valid("1HGCM82633A004352"));

// Inválido: contém a letra proibida 'I'
assert!(!vin::is_valid("1HGCM8263IA004352"));
```

#### Formatação e Conversão de Moeda

```rust
use brazilian_utils::currency;

// Formatar como moeda (sem símbolo, conforme a referência do contrato)
let formatado = currency::format_currency(1234.56);
assert_eq!(formatado, Some("1.234,56".to_string()));

// Converter para texto
let texto = currency::convert_real_to_text(1234.56);
assert_eq!(texto, "mil duzentos e trinta e quatro reais e cinquenta e seis centavos");

// Converter um valor em texto para número
assert_eq!(currency::parse("R$ 1.234,56"), 1234.56);
```

#### Número por Extenso

```rust
use brazilian_utils::number::{convert_to_words, ConvertNumberToWordsOptions};

// Escrever por extenso
assert_eq!(convert_to_words(1235.0, None), "mil duzentos e trinta e cinco");

// Concordância no feminino
assert_eq!(
    convert_to_words(2.0, Some(ConvertNumberToWordsOptions { feminine: Some(true) })),
    "duas"
);
```

#### Texto (Capitalização e Remoção de Acentos)

```rust
use brazilian_utils::text;

// Remover acentos
assert_eq!(text::remove_accents("São Paulo"), "Sao Paulo");

// Capitalizar (preposições em minúsculas, siglas em maiúsculas)
assert_eq!(text::capitalize("fulano de tal", None), "Fulano de Tal");
assert_eq!(text::capitalize("empresa ltda", None), "Empresa LTDA");
```

#### Validação de Número de Telefone

```rust
use brazilian_utils::phone;

// Validar (aceita celular ou fixo)
assert!(phone::is_valid("11994029275", None));
assert!(phone::is_valid("1635014415", None));

// Validar especificamente celular ou fixo
assert!(phone::is_valid_mobile("11994029275", None));
assert!(phone::is_valid_landline("1635014415"));

// Formatar (máscara do número assinante, XXXXX-XXXX)
assert_eq!(phone::format_phone("988887777"), "98888-7777");

// Gerar aleatório
let numero_telefone = phone::generate(Some("mobile"));
assert!(phone::is_valid(&numero_telefone, None));
```

#### DDD (Código de Área)

```rust
use brazilian_utils::area_code;

// Consultar UF e região de um DDD
let info = area_code::get_info(11).unwrap();
assert_eq!(info.state_code, "SP");

// DDD que atende mais de um estado
let df = area_code::get_info(61).unwrap();
assert_eq!(df.state_codes, vec!["DF".to_string(), "GO".to_string()]);

// Listar os DDDs de um estado
assert_eq!(area_code::list_by_state("AC"), vec![68]);
```

#### Município

```rust
use brazilian_utils::municipality;

// Buscar pelo código IBGE
let sp = municipality::get_by_code("3550308").unwrap();
assert_eq!(sp.name, "São Paulo");
assert_eq!(sp.state_code, "SP");

// Listar municípios de um estado
let df = municipality::list(Some("DF"));
assert_eq!(df.len(), 1);
```

#### Estado (UF)

```rust
use brazilian_utils::state;

// Nome a partir da sigla
assert_eq!(state::get_name_by_code("sp"), Some("São Paulo".to_string()));

// Fuso horário
assert_eq!(state::get_timezone("AM"), Some("America/Manaus".to_string()));
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
use brazilian_utils::legal_process::{self, GenerateProcessoJuridicoParams};

// Validar
assert!(legal_process::is_valid("00020802520125150049"));

// Formatar
let formatado = legal_process::format_legal_process("00020802520125150049");
assert_eq!(formatado, Some("0002080-25.2012.5.15.0049".to_string()));

// Gerar aleatório para um órgão específico
let processo = legal_process::generate(Some(GenerateProcessoJuridicoParams {
    year: None,
    court: Some(5),
}));
assert!(legal_process::is_valid(&processo.unwrap()));
```

#### Registro Profissional (OAB, CRM, CRO, CRP, CRC)

```rust
use brazilian_utils::registro_profissional::{is_valid, IsValidRegistroProfissionalParams};

// Validar OAB
assert!(is_valid(&IsValidRegistroProfissionalParams {
    value: "123456/SP".to_string(),
    council: "OAB".to_string(),
    state: None,
}));

// Validar CRC (com tipo de registro e dígito)
assert!(is_valid(&IsValidRegistroProfissionalParams {
    value: "SP-123456/O-3".to_string(),
    council: "CRC".to_string(),
    state: Some("SP".to_string()),
}));
```

#### Passaporte

```rust
use brazilian_utils::passport;

// Validar
assert!(passport::is_valid("AA111111"));

// Gerar aleatório
let numero_passaporte = passport::generate();
assert!(passport::is_valid(&numero_passaporte));
```

#### CNS (Cartão Nacional de Saúde)

```rust
use brazilian_utils::cns;

// Validar
assert!(cns::is_valid("123456789010000"));

// Formatar
assert_eq!(cns::format("123456789010001"), "123 4567 8901 0001");
```

#### Certidão de Registro Civil

```rust
use brazilian_utils::certidao;

// Validar matrícula (art. 473 do CNNCNJ)
assert!(certidao::is_valid("104539 01 55 2013 1 00012 021 0000123 21"));

// Extrair campos
let info = certidao::get_info("104539 01 55 2013 1 00012 021 0000123 21").unwrap();
assert_eq!(info.book_type, "birth");
```

#### CAEPF, CEI e CNO

```rust
use brazilian_utils::{caepf, cei, cno};

// CAEPF: validar e formatar
assert!(caepf::is_valid("293.118.610/001-84"));
assert_eq!(caepf::format("29311861000184"), "293.118.610/001-84");

// CEI: validar e formatar
assert!(cei::is_valid("11.583.00249/85"));
assert_eq!(cei::format("277297118187"), "27.729.71181/87");

// CNO: validar e formatar
assert!(cno::is_valid("110840168062"));
assert_eq!(cno::format("111130137368"), "11.113.01373/68");
```

#### CFOP, CNAE, CSOSN, CST e NCM (Códigos Fiscais)

```rust
use brazilian_utils::{cfop, cnae, csosn, cst, ncm};

// CFOP: validar e consultar descrição
assert!(cfop::is_valid("5102"));
let cfop_info = cfop::get("5102").unwrap();
assert!(cfop_info.description.starts_with("Venda de mercadoria"));

// CNAE: validar, formatar e consultar
assert!(cnae::is_valid("6201501"));
assert_eq!(cnae::format("6201501"), "6201-5/01");

// CSOSN e CST: validar
assert!(csosn::is_valid("101"));
assert!(cst::is_valid("110", None));

// NCM: validar e formatar
assert!(ncm::is_valid("22030000"));
assert_eq!(ncm::format("84713012"), "8471.30.12");
```

#### CBO (Classificação Brasileira de Ocupações)

```rust
use brazilian_utils::cbo;

// Validar
assert!(cbo::is_valid("212405"));

// Consultar descrição oficial
let ocupacao = cbo::get("212405").unwrap();
assert_eq!(ocupacao.description, "Analista de desenvolvimento de sistemas");
```

#### Chave de Acesso da NF-e

```rust
use brazilian_utils::nfe_key;

// Validar chave de acesso (44 dígitos)
assert!(nfe_key::is_valid("35170458716523000119550010000000121000123458"));

// Extrair campos
let info = nfe_key::get_info("35170458716523000119550010000000121000123458").unwrap();
assert_eq!(info.state_code, "SP");
assert_eq!(info.year, 2017);
```

#### Inscrição Estadual (IE)

```rust
use brazilian_utils::ie;

// Validar estrutura para o estado informado
assert!(ie::is_valid("123456789012", "SP"));

// RJ espera 8 dígitos, não 12
assert!(!ie::is_valid("123456789012", "RJ"));
```

#### Banco, Conta Bancária e IBAN

```rust
use brazilian_utils::{bank, bank_account, iban};
use brazilian_utils::bank_account::IsValidBankAccountParams;

// Banco: buscar por código COMPE ou ISPB
let banco = bank::get_by_code("1").unwrap();
assert_eq!(banco.name, "Banco do Brasil S.A.");

// Conta bancária: validação genérica (módulo 10 ou 11)
let params = IsValidBankAccountParams {
    bank_code: "001".to_string(),
    agency: "1234".to_string(),
    account: "123456".to_string(),
    digit: "6".to_string(),
};
assert!(bank_account::is_valid(&params));

// IBAN: validar e formatar
assert!(iban::is_valid("BR1500000000000010932840814P2"));
assert_eq!(
    iban::format("BR1500000000000010932840814P2"),
    "BR15 0000 0000 0000 1093 2840 814P 2"
);
```

#### Chave Pix e Payload Pix (BR Code)

```rust
use brazilian_utils::pix_key;
use brazilian_utils::pix_payload::{generate, is_valid, GeneratePixPayloadParams};

// Identificar e validar uma chave Pix
let info = pix_key::get_info("123.456.789-09").unwrap();
assert_eq!(info.key_type, "cpf");
assert_eq!(info.value, "12345678909");

// Gerar um payload (BR Code) estático
let payload = generate(GeneratePixPayloadParams {
    key: Some("123e4567-e12b-12d1-a456-426655440000".to_string()),
    merchant_name: "Fulano de Tal".to_string(),
    merchant_city: "Brasilia".to_string(),
    ..Default::default()
}).unwrap();
assert!(is_valid(&payload));
```

#### Cartão de Crédito/Débito

```rust
use brazilian_utils::credit_card;

// Validar (algoritmo de Luhn)
assert!(credit_card::is_valid("4111111111111111"));
assert!(!credit_card::is_valid("4111111111111112"));
```

#### Utilitários de Data

```rust
use brazilian_utils::date_utils::{self, IsHolidayParams};
use chrono::NaiveDate;

// Verificar se é feriado nacional
let natal = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
assert_eq!(
    date_utils::is_holiday(Some(IsHolidayParams { date: Some(natal), uf: None })),
    Some(true)
);

// Converter data para texto
assert_eq!(
    date_utils::convert_date_to_text("01/01/2024"),
    Some("Primeiro de janeiro de dois mil e vinte e quatro".to_string())
);

// Verificar dia útil e listar os feriados do ano
assert!(!date_utils::is_business_day(natal, None));
assert!(date_utils::get_holidays(2024).iter().any(|h| h.name == "Natal"));
```

### Todos os Módulos Disponíveis

| Módulo | Funções | Descrição |
|--------|---------|-----------|  
| `boleto` | `is_valid`, `validate` | Validação de linha digitável de boleto |
| `cep` | `is_valid`, `format_cep`, `remove_symbols`, `generate`, `get_address_from_cep`, `get_cep_information_from_address` | Validação de CEP e busca de endereço |
| `cnh` | `is_valid_cnh` | Validação de CNH |
| `cnpj` | `is_valid`, `validate`, `format_cnpj`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Validação de registro empresarial |
| `cpf` | `is_valid`, `validate`, `format_cpf`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Validação de CPF |
| `currency` | `format_currency`, `convert_real_to_text`, `number_to_words` | Formatação e conversão de moeda |
| `date_utils` | `is_holiday`, `convert_date_to_text` | Utilitários de data e verificação de feriados |
| `email` | `is_valid` | Validação de email RFC 5322 |
| `legal_nature` | `is_valid`, `get_description`, `list_all` | Códigos de natureza jurídica (92 códigos, Natureza Jurídica 2021) |
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

### Agradecimentos

Inspirado em [brazilian-utils/python](https://github.com/brazilian-utils/python) - Uma biblioteca Python com utilitários similares para dados brasileiros.

---

## English

A Rust library providing utility functions for Brazilian-specific data validation, formatting, and generation.

### Features

This library provides comprehensive utilities for handling Brazilian documents, identifiers, and data formats:

#### 📋 Document Validation & Formatting

- **CPF** (Cadastro de Pessoas Físicas) - Individual Taxpayer Registry
- **CNPJ** (Cadastro Nacional da Pessoa Jurídica) - National Registry of Legal Entities, including the new alphanumeric format (IN RFB nº 2.119/2021)
- **CNH** (Carteira Nacional de Habilitação) - National Driver's License
- **PIS** (Programa de Integração Social) - Social Integration Program
- **Voter ID** (Título de Eleitor) - Electoral Registration

#### 🆔 Additional Documents & Registries

- **Passport** - Validation and generation of a Brazilian passport number
- **CNS** (Cartão Nacional de Saúde) - Validation and formatting of the national health card
- **Civil Registry Certificate** (Certidão) - Validation and parsing of the record number (birth, marriage, death, etc., art. 473 of the CNNCNJ)
- **CAEPF** (individual taxpayer's economic activity registry) - Validation and formatting
- **CEI** (INSS-specific registry) - Validation and formatting
- **CNO** (National Registry of Construction Works) - Validation and formatting

#### 🚗 Vehicle & Transportation

- **License Plate** - Old and Mercosul format validation and conversion
- **RENAVAM** (Registro Nacional de Veículos Automotores) - National Motor Vehicle Registry
- **VIN** (Vehicle Identification Number / chassi) - Structural validation with the North American check digit

#### 🏛️ Legal & Administrative

- **Legal Process** - Brazilian legal system process numbers
- **Legal Nature** - Legal entity classification (60+ official codes)
- **Professional Registration** - Structural validation of OAB, CRM, CRO, CRP and CRC numbers

#### 📊 Tax & Official Classification Codes

- **CFOP** (Código Fiscal de Operações e Prestações) - Validation and official description lookup
- **CNAE** (National Classification of Economic Activities) - Validation, formatting and lookup
- **CSOSN** (Simples Nacional operation status code) - Validation of the 10 official codes
- **CST** (Tax Situation Code) - Validation of ICMS, IPI, PIS and COFINS codes
- **NCM** (Mercosul Common Nomenclature) - Validation and formatting of customs codes
- **NF-e Access Key** - Validation and parsing of the 44-digit key (NF-e, NFC-e, CT-e, MDF-e and other DF-e)
- **State Tax Registration (IE)** - Structural validation per state
- **CBO** (Brazilian Occupation Classification) - Validation and official description lookup

#### 💳 Banking & Payments

- **Bank** - Lookup by COMPE code or ISPB
- **Bank Account** - Structural validation of agency, account and check digit
- **IBAN** - Validation, formatting and parsing of the Brazilian IBAN
- **Pix Key** - Identification and validation of CPF, CNPJ, email, phone and EVP keys
- **Pix Payload** (BR Code) - Validation, parsing and generation of "Pix copia e cola"
- **Credit/Debit Card** - Validation via the Luhn algorithm

#### 📍 Location & Communication

- **CEP** (Código de Endereçamento Postal) - Postal Code with address lookup
- **Phone** - Mobile and landline validation with formatting
- **Area Code** (DDD) - State and region lookup by area code
- **Municipality** - Lookup by IBGE code and listing by state
- **State (UF)** - Name, code, timezone lookup and listing

#### 💰 Financial & Text

- **Boleto** - Bank slip digitable line validation
- **Currency** - Real (BRL) formatting and text conversion
- **Date Utils** - Holiday checking (fixed, movable and state), business days and date text conversion
- **Email** - RFC 5322 compliant validation
- **Number to Words** - Converts numbers into text, with grammatical gender agreement
- **Text** - Name/company capitalization and accent removal

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
assert!(cnpj::is_valid("03560714000142", None));

// Format
let formatted = cnpj::format_cnpj("03560714000142");
assert_eq!(formatted, Some("03.560.714/0001-42".to_string()));

// Generate random valid CNPJ
let cnpj_number = cnpj::generate(None, None);
assert!(cnpj::is_valid(&cnpj_number, None));

// New alphanumeric format (v2, IN RFB nº 2.119/2021)
let cnpj_v2 = cnpj::generate(None, Some(2));
assert!(cnpj::is_valid(&cnpj_v2, Some(2)));
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

#### Boleto Validation

```rust
use brazilian_utils::boleto;

// Validate boleto digitable line
assert!(boleto::is_valid("00190000090114971860168524522114675860000102656"));

// Validate with formatting (spaces and dots)
assert!(boleto::is_valid("0019000009 01149.718601 68524.522114 6 75860000102656"));

// Validate invalid line
assert!(!boleto::is_valid("00190000020114971860168524522114675860000102656"));
```

#### License Plate (Old and Mercosul)

```rust
use brazilian_utils::license_plate;

// Validate (accepts either the old or the Mercosul format)
assert!(license_plate::is_valid("ABC1234"));
assert!(license_plate::is_valid("ABC1D23"));

// Validate a specific format
assert!(license_plate::is_valid_for_format("ABC1234", Some("old_format")));
assert!(license_plate::is_valid_for_format("ABC1D23", Some("mercosul")));

// Convert old to Mercosul
let mercosul = license_plate::convert_to_mercosul("ABC1234");
assert_eq!(mercosul, "ABC1C34".to_string());

// Format
let formatted = license_plate::format_license_plate("ABC1234");
assert_eq!(formatted, Some("ABC-1234".to_string()));
```

#### VIN (Vehicle Identification Number / chassi)

```rust
use brazilian_utils::vin;

// Validate (North American check digit)
assert!(vin::is_valid("1HGCM82633A004352"));

// Invalid: contains the excluded letter 'I'
assert!(!vin::is_valid("1HGCM8263IA004352"));
```

#### Currency Formatting and Text Conversion

```rust
use brazilian_utils::currency;

// Format as currency (no symbol, matching the contract's reference)
let formatted = currency::format_currency(1234.56);
assert_eq!(formatted, Some("1.234,56".to_string()));

// Convert to text
let text = currency::convert_real_to_text(1234.56);
assert_eq!(text, "mil duzentos e trinta e quatro reais e cinquenta e seis centavos");

// Parse a BRL amount string back into a number
assert_eq!(currency::parse("R$ 1.234,56"), 1234.56);
```

#### Number to Words

```rust
use brazilian_utils::number::{convert_to_words, ConvertNumberToWordsOptions};

// Write a number out in words
assert_eq!(convert_to_words(1235.0, None), "mil duzentos e trinta e cinco");

// Feminine grammatical agreement
assert_eq!(
    convert_to_words(2.0, Some(ConvertNumberToWordsOptions { feminine: Some(true) })),
    "duas"
);
```

#### Text (Capitalization and Accent Removal)

```rust
use brazilian_utils::text;

// Remove accents
assert_eq!(text::remove_accents("São Paulo"), "Sao Paulo");

// Capitalize (prepositions stay lower case, designations stay upper case)
assert_eq!(text::capitalize("fulano de tal", None), "Fulano de Tal");
assert_eq!(text::capitalize("empresa ltda", None), "Empresa LTDA");
```

#### Phone Number Validation

```rust
use brazilian_utils::phone;

// Validate (accepts mobile or landline)
assert!(phone::is_valid("11994029275", None));
assert!(phone::is_valid("1635014415", None));

// Validate specifically as mobile or landline
assert!(phone::is_valid_mobile("11994029275", None));
assert!(phone::is_valid_landline("1635014415"));

// Format (subscriber-number mask, XXXXX-XXXX)
assert_eq!(phone::format_phone("988887777"), "98888-7777");

// Generate random
let phone_number = phone::generate(Some("mobile"));
assert!(phone::is_valid(&phone_number, None));
```

#### Area Code (DDD)

```rust
use brazilian_utils::area_code;

// Look up the state and region of a DDD
let info = area_code::get_info(11).unwrap();
assert_eq!(info.state_code, "SP");

// A DDD that straddles two states
let df = area_code::get_info(61).unwrap();
assert_eq!(df.state_codes, vec!["DF".to_string(), "GO".to_string()]);

// List the DDDs of a state
assert_eq!(area_code::list_by_state("AC"), vec![68]);
```

#### Municipality

```rust
use brazilian_utils::municipality;

// Look up by IBGE code
let sp = municipality::get_by_code("3550308").unwrap();
assert_eq!(sp.name, "São Paulo");
assert_eq!(sp.state_code, "SP");

// List the municipalities of a state
let df = municipality::list(Some("DF"));
assert_eq!(df.len(), 1);
```

#### State (UF)

```rust
use brazilian_utils::state;

// Name from the state code
assert_eq!(state::get_name_by_code("sp"), Some("São Paulo".to_string()));

// Timezone
assert_eq!(state::get_timezone("AM"), Some("America/Manaus".to_string()));
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
use brazilian_utils::legal_process::{self, GenerateProcessoJuridicoParams};

// Validate
assert!(legal_process::is_valid("00020802520125150049"));

// Format
let formatted = legal_process::format_legal_process("00020802520125150049");
assert_eq!(formatted, Some("0002080-25.2012.5.15.0049".to_string()));

// Generate random, for a specific segment (court)
let process = legal_process::generate(Some(GenerateProcessoJuridicoParams {
    year: None,
    court: Some(5),
}));
assert!(legal_process::is_valid(&process.unwrap()));
```

#### Professional Registration (OAB, CRM, CRO, CRP, CRC)

```rust
use brazilian_utils::registro_profissional::{is_valid, IsValidRegistroProfissionalParams};

// Validate an OAB number
assert!(is_valid(&IsValidRegistroProfissionalParams {
    value: "123456/SP".to_string(),
    council: "OAB".to_string(),
    state: None,
}));

// Validate a CRC number (with registration type and digit)
assert!(is_valid(&IsValidRegistroProfissionalParams {
    value: "SP-123456/O-3".to_string(),
    council: "CRC".to_string(),
    state: Some("SP".to_string()),
}));
```

#### Passport

```rust
use brazilian_utils::passport;

// Validate
assert!(passport::is_valid("AA111111"));

// Generate a random one
let passport_number = passport::generate();
assert!(passport::is_valid(&passport_number));
```

#### CNS (National Health Card)

```rust
use brazilian_utils::cns;

// Validate
assert!(cns::is_valid("123456789010000"));

// Format
assert_eq!(cns::format("123456789010001"), "123 4567 8901 0001");
```

#### Civil Registry Certificate (Certidão)

```rust
use brazilian_utils::certidao;

// Validate the record number (art. 473 of the CNNCNJ)
assert!(certidao::is_valid("104539 01 55 2013 1 00012 021 0000123 21"));

// Parse its fields
let info = certidao::get_info("104539 01 55 2013 1 00012 021 0000123 21").unwrap();
assert_eq!(info.book_type, "birth");
```

#### CAEPF, CEI and CNO

```rust
use brazilian_utils::{caepf, cei, cno};

// CAEPF: validate and format
assert!(caepf::is_valid("293.118.610/001-84"));
assert_eq!(caepf::format("29311861000184"), "293.118.610/001-84");

// CEI: validate and format
assert!(cei::is_valid("11.583.00249/85"));
assert_eq!(cei::format("277297118187"), "27.729.71181/87");

// CNO: validate and format
assert!(cno::is_valid("110840168062"));
assert_eq!(cno::format("111130137368"), "11.113.01373/68");
```

#### CFOP, CNAE, CSOSN, CST and NCM (Tax Codes)

```rust
use brazilian_utils::{cfop, cnae, csosn, cst, ncm};

// CFOP: validate and look up the description
assert!(cfop::is_valid("5102"));
let cfop_info = cfop::get("5102").unwrap();
assert!(cfop_info.description.starts_with("Venda de mercadoria"));

// CNAE: validate, format and look up
assert!(cnae::is_valid("6201501"));
assert_eq!(cnae::format("6201501"), "6201-5/01");

// CSOSN and CST: validate
assert!(csosn::is_valid("101"));
assert!(cst::is_valid("110", None));

// NCM: validate and format
assert!(ncm::is_valid("22030000"));
assert_eq!(ncm::format("84713012"), "8471.30.12");
```

#### CBO (Brazilian Occupation Classification)

```rust
use brazilian_utils::cbo;

// Validate
assert!(cbo::is_valid("212405"));

// Look up the official description
let occupation = cbo::get("212405").unwrap();
assert_eq!(occupation.description, "Analista de desenvolvimento de sistemas");
```

#### NF-e Access Key

```rust
use brazilian_utils::nfe_key;

// Validate the 44-digit access key
assert!(nfe_key::is_valid("35170458716523000119550010000000121000123458"));

// Parse its fields
let info = nfe_key::get_info("35170458716523000119550010000000121000123458").unwrap();
assert_eq!(info.state_code, "SP");
assert_eq!(info.year, 2017);
```

#### State Tax Registration (IE)

```rust
use brazilian_utils::ie;

// Validate the structure for the given state
assert!(ie::is_valid("123456789012", "SP"));

// RJ expects 8 digits, not 12
assert!(!ie::is_valid("123456789012", "RJ"));
```

#### Bank, Bank Account and IBAN

```rust
use brazilian_utils::{bank, bank_account, iban};
use brazilian_utils::bank_account::IsValidBankAccountParams;

// Bank: look up by COMPE code or ISPB
let bank_info = bank::get_by_code("1").unwrap();
assert_eq!(bank_info.name, "Banco do Brasil S.A.");

// Bank account: generic validation (modulus 10 or 11)
let params = IsValidBankAccountParams {
    bank_code: "001".to_string(),
    agency: "1234".to_string(),
    account: "123456".to_string(),
    digit: "6".to_string(),
};
assert!(bank_account::is_valid(&params));

// IBAN: validate and format
assert!(iban::is_valid("BR1500000000000010932840814P2"));
assert_eq!(
    iban::format("BR1500000000000010932840814P2"),
    "BR15 0000 0000 0000 1093 2840 814P 2"
);
```

#### Pix Key and Pix Payload (BR Code)

```rust
use brazilian_utils::pix_key;
use brazilian_utils::pix_payload::{generate, is_valid, GeneratePixPayloadParams};

// Identify and validate a Pix key
let info = pix_key::get_info("123.456.789-09").unwrap();
assert_eq!(info.key_type, "cpf");
assert_eq!(info.value, "12345678909");

// Generate a static payload (BR Code)
let payload = generate(GeneratePixPayloadParams {
    key: Some("123e4567-e12b-12d1-a456-426655440000".to_string()),
    merchant_name: "Fulano de Tal".to_string(),
    merchant_city: "Brasilia".to_string(),
    ..Default::default()
}).unwrap();
assert!(is_valid(&payload));
```

#### Credit/Debit Card

```rust
use brazilian_utils::credit_card;

// Validate (Luhn algorithm)
assert!(credit_card::is_valid("4111111111111111"));
assert!(!credit_card::is_valid("4111111111111112"));
```

#### Date Utilities

```rust
use brazilian_utils::date_utils::{self, IsHolidayParams};
use chrono::NaiveDate;

// Check if a date is a national holiday
let christmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
assert_eq!(
    date_utils::is_holiday(Some(IsHolidayParams { date: Some(christmas), uf: None })),
    Some(true)
);

// Convert date to text
assert_eq!(
    date_utils::convert_date_to_text("01/01/2024"),
    Some("Primeiro de janeiro de dois mil e vinte e quatro".to_string())
);

// Check business days and list a year's holidays
assert!(!date_utils::is_business_day(christmas, None));
assert!(date_utils::get_holidays(2024).iter().any(|h| h.name == "Natal"));
```

### All Available Modules

| Module | Functions | Description |
|--------|-----------|-------------|
| `boleto` | `is_valid`, `validate` | Bank slip digitable line validation |
| `cep` | `is_valid`, `format_cep`, `remove_symbols`, `generate`, `get_address_from_cep`, `get_cep_information_from_address` | Postal code validation and address lookup |
| `cnh` | `is_valid_cnh` | Driver's license validation |
| `cnpj` | `is_valid`, `validate`, `format_cnpj`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Company registration validation |
| `cpf` | `is_valid`, `validate`, `format_cpf`, `remove_symbols`, `generate`, `hashdigit`, `compute_checksum` | Individual taxpayer validation |
| `currency` | `format_currency`, `convert_real_to_text`, `number_to_words` | Currency formatting and text conversion |
| `date_utils` | `is_holiday`, `convert_date_to_text` | Date utilities and holiday checking |
| `email` | `is_valid` | RFC 5322 email validation |
| `legal_nature` | `is_valid`, `get_description`, `list_all` | Legal entity nature codes (92 codes, Natureza Jurídica 2021) |
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

## Acknowledgments

Inspired by [brazilian-utils/python](https://github.com/brazilian-utils/python) - A Python library with similar utilities for Brazilian data.