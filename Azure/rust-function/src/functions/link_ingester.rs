use azure_functions::{
    bindings::QueueTrigger,
    func,
};

use musicfunc::helper::link_parser::LinkType;


#[func]
#[binding(name = "trigger", queue_name = "linkqueue")]
pub fn link_ingester(trigger: QueueTrigger) {

    if let Some(linkType) = trigger.message.parse::<LinkType>() {
        match linkType {
            LinkType::Album(uri) => GetAlbumMetadata(uri).await,

        }

    }

}
