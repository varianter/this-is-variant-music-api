use crate::helper::link_parser::LinkType;
use std::error::Error;
use aspotify::{Client, ClientCredentials};


async fn get_metadata(link_type: LinkType) -> Result<(), Error> {
    let credentials = ClientCredentials::from_env()
        .expect("CLIENT_ID and CLIENT_SECRET not found.");
    let client = Client::new(credentials);

    match link_type {
        LinkType::SpotifyAlbum(uri) => {
            client.albums().get_album_tracks(uri, None).await.unwrap()
        }
        _ => Err("Enum not implemented".into())
    }
}
