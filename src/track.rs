// Copyright (C) 2023 Koen Bolhuis
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use mpris::Metadata;

#[derive(Debug, Default, PartialEq)]
pub struct Track {
    artist: String,
    title: String,
    album: Option<String>,
    length: Option<u128>,
}

impl Track {
    pub fn artist(&self) -> &str {
        &self.artist
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn album(&self) -> Option<&str> {
        self.album.as_deref()
    }

    pub fn length(&self) -> Option<u128> {
        self.length
    }

    pub fn new(artist: &str, title: &str, album: Option<&str>, length: Option<u128>) -> Self {
        Self {
            artist: artist.to_owned(),
            title: title.to_owned(),
            album: album.and_then(|album| {
                if !album.is_empty() {
                    Some(album.to_owned())
                } else {
                    None
                }
            }),
            length: length,
        }
    }

    pub fn clear(&mut self) {
        self.artist.clear();
        self.title.clear();
        self.album.take();
    }

    pub fn clone_from(&mut self, other: &Self) {
        self.artist.clone_from(&other.artist);
        self.title.clone_from(&other.title);
        self.album.clone_from(&other.album);
        self.length.clone_from(&other.length);
    }

    pub fn from_metadata(metadata: &Metadata) -> Self {
        let artist = metadata
            .artists()
            .as_ref()
            .and_then(|artists| artists.first().copied())
            .unwrap_or("")
            .to_owned();

        let title = metadata.title().unwrap_or("").to_owned();

        let album = metadata.album_name().and_then(|album| {
            if !album.is_empty() {
                Some(album.to_owned())
            } else {
                None
            }
        });

        let length: Option<u128> = match metadata.length() {
            Some(d) => Some(d.as_millis().to_owned()),
            None => None,
        };

        Self {
            artist,
            title,
            album,
            length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpris::MetadataValue;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        // Constructing a track with an empty album should result in `None` for `Track::album()`

        assert_eq!(
            Track::new("Enter Shikari", "Live Outside", None, Some(220)).album(),
            None
        );

        // Constructing a track with a nonempty album should result in `Some` for `Track::album()`

        assert_eq!(
            Track::new("Dimension", "Psycho", Some("Organ"), Some(161)).album(),
            Some("Organ")
        );

        // Constructing a track with digits should result in `Some` for `Track::length()`

        assert_eq!(
            Track::new(
                "Nine Inch Nails",
                "Closer",
                Some("The Downward Spiral"),
                Some(373000)
            )
            .length(),
            Some(373_000 as u128)
        );
    }

    #[test]
    fn test_from_metadata() {
        // Metadata without an album should result in a `None` for `Track::album()`

        let mut metadata_without_album = HashMap::new();
        metadata_without_album.insert(
            "xesam:artists".to_owned(),
            MetadataValue::Array(vec![MetadataValue::String("Billy Joel".to_owned())]),
        );
        metadata_without_album.insert(
            "xesam:title".to_owned(),
            MetadataValue::String("We didn't start the fire".to_owned()),
        );
        let metadata_without_album = Metadata::from(metadata_without_album);
        let track_without_album = Track::from_metadata(&metadata_without_album);

        assert_eq!(track_without_album.album(), None);

        // Metadata with an empty album should result in a `None` for `Track::album()`

        let mut metadata_empty_album = HashMap::new();
        metadata_empty_album.insert(
            "xesam:artist".to_owned(),
            MetadataValue::Array(vec![MetadataValue::String("The Prodigy".to_owned())]),
        );
        metadata_empty_album.insert(
            "xesam:title".to_owned(),
            MetadataValue::String("Wild Frontier".to_owned()),
        );
        metadata_empty_album.insert(
            "xesam:album".to_owned(),
            MetadataValue::String("".to_owned()),
        );
        let metadata_empty_album = Metadata::from(metadata_empty_album);
        let track_empty_album = Track::from_metadata(&metadata_empty_album);

        assert_eq!(track_empty_album.album(), None);

        // Metadata with a nonempty album should result in a `Some` for `Track::album()`

        let mut metadata_with_album = HashMap::new();
        metadata_with_album.insert(
            "xesam:artist".to_owned(),
            MetadataValue::Array(vec![MetadataValue::String("Men At Work".to_owned())]),
        );
        metadata_with_album.insert(
            "xesam:title".to_owned(),
            MetadataValue::String("Who Can It Be Now?".to_owned()),
        );
        metadata_with_album.insert(
            "xesam:album".to_owned(),
            MetadataValue::String("Business As Usual".to_owned()),
        );
        let metadata_with_album = Metadata::from(metadata_with_album);
        let track_with_album = Track::from_metadata(&metadata_with_album);

        assert_eq!(track_with_album.album(), Some("Business As Usual"));

        // Metadata with length should result in a `Some` for `Track::length()`

        let mut metadata_with_length = HashMap::new();
        metadata_with_length.insert("mpris:length".to_owned(), MetadataValue::U64(206_000_000));

        metadata_with_length.insert(
            "xesam:artist".to_owned(),
            MetadataValue::Array(vec![MetadataValue::String("On-lyne".to_owned())]),
        );

        metadata_with_length.insert(
            "xesam:title".to_owned(),
            MetadataValue::String("Running Late".to_owned()),
        );
        let metadata_with_length = Metadata::from(metadata_with_length);
        let track_with_length = Track::from_metadata(&metadata_with_length);

        assert_eq!(track_with_length.length, Some(206000 as u128));
    }
}
