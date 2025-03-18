use ferrugem::ferrugem;

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
