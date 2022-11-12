use crate::*;

pub async fn health_check(_req: HttpRequest) -> impl Responder{
    // can drop finish() for readability.
    return HttpResponse::Ok().finish();
}
