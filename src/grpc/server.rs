use tonic::{transport::Server, Request, Response, Status};

use crate::crawler::standalone;

// grpc related
use crawling::crawler_server::{Crawler, CrawlerServer};
use crawling::{CrawlingRequest, CrawlingReply, Frontier, Extract};

pub mod crawling {
    tonic::include_proto!("crawling");
}

#[derive(Debug, Default)]
struct CrawlingStruct {}

#[tonic::async_trait]
impl Crawler for CrawlingStruct {
    async fn crawl_url(
        &self,
        request: Request<CrawlingRequest>,
    ) -> Result<Response<CrawlingReply>, Status> {
        // TODO: make crawling errors recoverable
        let data = standalone::execute(request.get_ref().host.clone()).expect("could not crawl using distributed mode");
        let mut frontier: Vec<Extract> = vec![];

        for (domain, uri) in data {
            frontier.push(
                Extract{ domain, uri}
            );
        }

        let frontier_res = Some(Frontier {extracts: frontier});
        let res = CrawlingReply {frontier: frontier_res};

        Ok(Response::new(res))
    }
}

#[tokio::main]
pub async fn run_server(server_addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let addr = server_addr.parse()?;
    let crawler = CrawlingStruct::default();
    
    Server::builder()
        .add_service(CrawlerServer::new(crawler))
        .serve(addr)
        .await?;

    Ok(())
}
