use actix_web::{web, HttpResponse, Responder};
use crate::models::models::Votation; // Certifique-se de que `Votation` é importada

// Função principal para manipular a requisição POST
pub async fn handle_post_votation(body: web::Json<Votation>) -> impl Responder {
    let votation = body.into_inner();

    // Chama a função assíncrona para criar uma votação (QUANDO FOR IMPLEMENTAR SÓ FAZ AQUI MESMO, NÃO PRECISA FAZER NA FUNÇÃO DE TESTE)
    let result = create_votation(&votation).await;

    // Retorna uma resposta com base no resultado da criação da votação
    match result {
        Ok(_) => HttpResponse::Ok().body("Votation created"),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}

// Apenas retorna Ok para teste
async fn create_votation(_votation: &Votation) -> Result<(), String> {
    Ok(())
}
