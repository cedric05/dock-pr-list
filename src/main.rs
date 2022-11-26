mod cli;
use clap::Parser;
use dkregistry::v2::Client;
use futures_util::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    let host = args.registry.unwrap();
    let builder = Client::configure().insecure_registry(args.insecure);
    let is_auth_provided =
        !args.username.as_ref().unwrap().is_empty() && !args.password.as_ref().unwrap().is_empty();
    let builder = builder.username(args.username).password(args.password);
    let mut dclient = builder.registry(&host).build()?;

    let login_scope = format!("repository");
    if is_auth_provided {
        dclient = dclient
            .authenticate(&[&login_scope])
            .await
            .expect("authentication failed");
    }

    match args.command {
        cli::Commands::Ls { image } => {
            let mut tags_list = dclient.get_tags(&image, None).boxed();
            println!("list of tags are");
            loop {
                let tag = tags_list.next().await;
                match tag {
                    Some(Ok(tags)) => {
                        let digest = dclient.get_manifestref(&image, &tags).await;
                        let digest = digest.ok().flatten().unwrap_or("".to_string());
                        println!("\t{} {}", tags, digest);
                    }
                    Some(Err(error)) => {
                        println!("error is  {}", error);
                        break;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        cli::Commands::Search { image } => {
            let mut catalog = dclient.get_catalog(None).boxed();
            println!("list of repositieries");
            loop {
                let tag = catalog.next().await;
                match tag {
                    Some(Ok(repo)) => {
                        if repo.contains(&image) {
                            println!("\t{}", repo);
                        }
                    }
                    Some(Err(error)) => {
                        println!("error is  {}", error);
                        break;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    };

    Ok(())
}
