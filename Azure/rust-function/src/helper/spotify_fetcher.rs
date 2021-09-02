use crate::helper::link_parser::LinkType;
use aspotify::{Client, ClientCredentials, Error, Page, Response, TrackSimplified};

async fn get_metadata(link_type: LinkType<'_>) -> Result<Response<Page<TrackSimplified>>, Error> {
    let credentials =
        ClientCredentials::from_env().expect("CLIENT_ID and CLIENT_SECRET not found.");
    let client = Client::new(credentials);

    match link_type {
        LinkType::SpotifyAlbum(uri) => Ok(client
            .albums()
            .get_album_tracks(uri, 20, 0, None)
            .await
            .unwrap()),
        LinkType::SpotifyTrack(_) => todo!(),
        LinkType::SpotifyPlaylist(_) => todo!(),
    }
}
