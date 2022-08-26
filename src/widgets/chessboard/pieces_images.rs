use std::collections::HashMap;

use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;

use anyhow::{self, Context};

#[derive(Clone)]
pub(crate) struct PiecesImages {
    pub(crate) pixbufs: HashMap<char, Pixbuf>,
}

impl PiecesImages {
    pub(crate) fn new(size: i32) -> anyhow::Result<Self> {
        let streams = PiecesImages::build_streams();
        let pixbufs = PiecesImages::build_pixbufs(&streams, size)?;

        Ok(Self { pixbufs })
    }

    pub(crate) fn build_streams() -> HashMap<char, MemoryInputStream> {
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

    pub(crate) fn build_pixbufs(
        streams: &HashMap<char, MemoryInputStream>,
        size: i32,
    ) -> anyhow::Result<HashMap<char, Pixbuf>> {
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
            .with_context(|| "Failed to interpret image.")?;

            result.insert(*kind, pixbuf);
        }

        Ok(result)
    }

    pub(crate) fn get_piece_pixbuf(piece_type: char, size: i32) -> anyhow::Result<Pixbuf> {
        let piece_type_lowercase = piece_type.to_ascii_lowercase();
        if piece_type_lowercase == 'q'
            || piece_type_lowercase == 'r'
            || piece_type_lowercase == 'b'
            || piece_type_lowercase == 'n'
        {
            let data: &[u8] = match piece_type {
                'Q' => include_bytes!("./vectors/Chess_qlt45.svg"),
                'R' => include_bytes!("./vectors/Chess_rlt45.svg"),
                'B' => include_bytes!("./vectors/Chess_blt45.svg"),
                'N' => include_bytes!("./vectors/Chess_nlt45.svg"),
                'q' => include_bytes!("./vectors/Chess_qdt45.svg"),
                'r' => include_bytes!("./vectors/Chess_rdt45.svg"),
                'b' => include_bytes!("./vectors/Chess_bdt45.svg"),
                'n' => include_bytes!("./vectors/Chess_ndt45.svg"),
                _ => panic!("Forbidden piece type {}.", piece_type),
            };
            let data = data;
            let data = Bytes::from(data);
            let image_stream = MemoryInputStream::from_bytes(&data);

            let pixbuf = Pixbuf::from_stream_at_scale(
                &image_stream,
                size,
                size,
                true,
                None::<&gtk::gio::Cancellable>,
            )
            .with_context(|| "Failed to interpret image.")?;

            Ok(pixbuf)
        } else {
            Err(anyhow::anyhow!("Forbidden piece type {}", piece_type))
        }
    }
}
