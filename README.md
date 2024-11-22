# PJtizator

**PJtizator** é uma aplicação desenvolvida em Rust utilizando o framework [Leptos](https://leptos.dev/) para calcular e simular os custos associados ao regime de trabalho PJ (Pessoa Jurídica) e CLT no Brasil. Este projeto permite a comparação entre os regimes de tributação do Simples Nacional, calculando valores como INSS, IRRF, FGTS, entre outros.

## Funcionalidades

- **Cálculo de Salário Líquido**: Simulação do salário líquido de um profissional contratado via CLT, incluindo:

  - INSS
  - IRRF
  - FGTS
  - Provisão de férias e 13º salário
  - Descontos adicionais (Vale Transporte, Vale Refeição, etc.)
  - Despesas empresariais adicionais (Sistema S, RAT, etc.)

- **Cálculo de impostos**: Simulação dos custos de contratação via PJ, incluindo:

  - INSS
  - IRRF
  - FGTS
  - Simples Nacional

- **Comparação de Anexos**: Comparação dos regimes de tributação Anexo III e Anexo V do Simples Nacional.

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação de sistemas utilizada para a lógica da aplicação.
- **Leptos**: Framework front-end em Rust utilizado para a construção da interface do usuário.
- **WebAssembly**: O projeto é compilado para WebAssembly (WASM), permitindo que a aplicação seja executada diretamente no navegador.
- **UIkit**: Framework CSS utilizado para estilizar a interface da aplicação.

## Requisitos

- **Rust**: Versão 1.63 ou superior.
- **Trunk**: Ferramenta para compilar projetos em Rust para WebAssembly e servir a aplicação no navegado
- **npm**: Gerenciador de pacotes para instalar dependências e servir a aplicação.

## Instalação

1. Clone o repositório:

   ```bash
   git clone https://github.com/seu-usuario/pjtizator.git
   cd pjtizator
   ```

2. instele as dependências:

   ```bash
   cargo install trunk
   npm install --global serve
   ```
3. Rodar localmente:

   ```bash
   trunk serve --open
   ```

## Buildar o projeto
1. Compile o projeto:

   ```bash
   trunk build --release
   ```

2. Inicie o servidor:

   ```bash
   serve .
   ```

5. Acesse a aplicação em `http://localhost:8080`.

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues para discutir novos recursos, bugs ou enviar pull requests.

## Licença

Este projeto é licenciado sob a licença MIT.
