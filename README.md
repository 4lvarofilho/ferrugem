# ferrugem

![](https://github.com/4lvarofilho/ferrugem/blob/main/logo.png)

cê tá cansado de escrever código Rust em inglês? curte falar um "caralho" aqui e ali? quer tentar uma parada diferenciada, em uma linguagem exótica? quer trazer um _brazilian touch_ aos seus programas?

**ferrugem** (tradução direta de _Rust_) está aqui para salvar seu dia, pois permite que você escreva programas Rust em português, usando palavras-chave em português, nomes de funções em português e expressões brasileiras.

Isso foi projetado para ser usado como a linguagem de programação oficial para desenvolver o futuro sistema operacional soberano brasileiro. BRAZIL I AM

Se você é do governo brasileiro ou de qualquer outro governo que tem o português como língua oficial: estarei aguardando suas doações.

Você é de Portugal (ou de outro lugar)? foda-se, aqui é o brasil caralho

just kidding,
O Rust em português é totalmente compatível com o Rust em inglês, então você pode misturar os dois à vontade.

Aqui está um exemplo do que pode ser alcançado com Ferrugem:

### Implementação básica

```rust
ferrugem! {
    funcao principal() {
        escreva!("Exemplo de MapaHash:");
        exemplo_mapa();

        escreva!("\nExemplo de Vetor:");
        exemplo_vetor();
    }

    use std::collections::MapaHash;

    funcao exemplo_mapa() {
        seja mut mapa = MapaHash::novo();

        mapa.inserir("um".para_texto(), 1);
        mapa.inserir("dois".para_texto(), 2);
        mapa.inserir("três".para_texto(), 3);

        para chave em ["um", "dois", "quatro"].iter() {
            combine mapa.obter(*chave) {
                Algum(valor) => escreva!("Valor para '{}': {}", chave, valor),
                Nenhum => escreva!("Chave '{}' não encontrada", chave),
            }
        }

        escreva!("\nTodos os elementos do mapa:");
        para (chave, valor) em &mapa {
            escreva!("{}: {}", chave, valor);
        }
    }

    funcao exemplo_vetor() {
        seja mut numeros = Vetor::novo();

        para i em 0..5 {
            numeros.adicionar(i * 10);
        }

        escreva!("Elementos do vetor:");
        para (indice, numero) em numeros.iter().enumerate() {
            escreva!("numeros[{}] = {}", indice, numero);
        }

        seja pares: Vetor<_> = numeros.iter()
            .filter(|&n| n % 20 == 0)
            .cloned()
            .collect();

        escreva!("\nApenas números divisíveis por 20:");
        para n em pares {
            escreva!("{}", n);
        }
    }
}
```

### E TAMBÉM TEM SUPORTE PRA O TIPO DE MERDA QUE TODO MUNDO FALA!!!!!

```rust
caralho!("foda-se");
merda!("deu merda");
```

(também tem alguns outros easter eggs, mas pra isso vc vai ter que desvendar no código)

### mas pq vc faria isso?

* diversão
* eu tava entediado
* é uma mera piada
* deu vontade
* EU NUNCA FICARIA PRA TRÁS DE UM FRANCÊS

### agradecimento

obrigado [Benjamin Bouvier](https://github.com/bnjbvr) pela implementação original em francês.

### licença

Licensed under the MIT License 📄