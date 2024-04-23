use mongodb::{bson::oid::ObjectId, Client, options::ClientOptions, Collection};
use serde::{Serialize, Deserialize};
use bson::doc;
use futures_util::stream::StreamExt;  // Correção na importação

#[derive(Debug, Serialize, Deserialize)]
struct Pessoa {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    nome: String,
    curriculo: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("desafio_rust");
    let collection: Collection<Pessoa> = database.collection("pessoas");

    println!("Conexão estabelecida com sucesso!");

    // Exemplo de inserção
    let nova_pessoa = Pessoa {
        id: None,  // O MongoDB preencherá isso automaticamente
        nome: "João Silva".to_string(),
        curriculo: "Desenvolvedor de Software".to_string(),
    };
    let insert_result = collection.insert_one(nova_pessoa, None).await?;
    println!("Nova pessoa inserida com o ID {:?}", insert_result.inserted_id);

    // Buscar todos
    let mut cursor = collection.find(None, None).await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let pessoa: Pessoa = document;
                println!("{:?}", pessoa);
            },
            Err(e) => println!("Erro ao buscar documento: {}", e),
        }
    }

    // Buscar por nome
    let filter = doc! { "nome": "João Silva" };
    let pessoa = collection.find_one(Some(filter), None).await?;
    println!("Pessoa encontrada: {:?}", pessoa);

    // Atualizar documento
    let filter = doc! { "nome": "João Silva" };
    let update = doc! { "$set": { "curriculo": "Engenheiro de Software" } };
    let update_result = collection.update_one(filter, update, None).await?;
    println!("Documentos atualizados: {}", update_result.modified_count);

    // Excluir documento
    let filter = doc! { "nome": "João Silva" };
    let delete_result = collection.delete_one(filter, None).await?;
    println!("Documentos excluídos: {}", delete_result.deleted_count);

    Ok(())
}
