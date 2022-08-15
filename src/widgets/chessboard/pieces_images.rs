use std::collections::HashMap;

use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;

#[derive(Clone)]
pub struct PiecesImages {
    pub pixbufs: HashMap<char, Pixbuf>,
}


impl PiecesImages {
    pub fn new(size: i32) -> Self {
        let streams = PiecesImages::build_streams();
        let pixbufs = PiecesImages::build_pixbufs(&streams, size);

        Self { pixbufs }
    }

    pub fn build_streams() -> HashMap<char, MemoryInputStream> {
        let mut result = HashMap::new();
        let pieces_types = vec!['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];
        let svg_defs: Vec<&[u8]> = vec![
            include_bytes!("./vectors/Chess_plt45.svg"),
            include_bytes!("./vectors/Chess_nlt45.svg"),
            include_bytes!("./vectors/Chess_blt45.svg"),
            include_bytes!("./vectors/Chess_rlt45.svg"),
            include_bytes!("./vectors/Chess_qlt45.svg"),
            include_bytes!("./vectors/Chess_klt45.svg"),
            include_bytes!("./vectors/Chess_pdt45.svg"),
            include_bytes!("./vectors/Chess_ndt45.svg"),
            include_bytes!("./vectors/Chess_bdt45.svg"),
            include_bytes!("./vectors/Chess_rdt45.svg"),
            include_bytes!("./vectors/Chess_qdt45.svg"),
            include_bytes!("./vectors/Chess_kdt45.svg"),
        ];
        let pieces_refs: Vec<(_, _)> = pieces_types.iter().zip(svg_defs.iter()).collect();

        for (kind, data) in pieces_refs.iter() {
            let kind = **kind;
            let image_data = **data;

            let image_data = Bytes::from(image_data);
            let image_stream = MemoryInputStream::from_bytes(&image_data);

            result.insert(kind, image_stream);
        }

        result
    }

    pub fn build_pixbufs(
        streams: &HashMap<char, MemoryInputStream>,
        size: i32,
    ) -> HashMap<char, Pixbuf> {
        let mut result = HashMap::new();

        for kind in streams.keys() {
            let image_stream = &streams[kind];

            let pixbuf = Pixbuf::from_stream_at_scale(
                image_stream,
                size,
                size,
                true,
                None::<&gtk::gio::Cancellable>,
            )
            .expect("Failed to interpret image.");

            result.insert(*kind, pixbuf);
        }

        result
    }
}
