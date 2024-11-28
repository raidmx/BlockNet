use binary::{Encode, v64, w32, Writer, Decode, Reader};
use crate::types::BlockPos;
use derive::Packet;
use crate::types::colour::VarRGBA;
use crate::types::map::{MapDecoration, MapTrackedObject, MapUpdateFlag};
    
/// Sent by the server to the client to update the data of a map shown to the client. It is sent
/// with a combination of flags that specify what data is updated. It may be used to update specific
/// parts of the map only. It is not required to send the full map each time when updating one part.
#[derive(Debug, Clone, Default, Packet)]
pub struct ClientBoundMapItemData {
    /// The unique identifier that represents the map that is updated over network. It remains
    /// consistent across sessions.
    pub map_id: v64,
    /// A combination of flags found above that indicate what parts of the map should be updated
    /// client-side.
    pub update_flags: w32,
    /// The dimension of the map that should be updated.
    pub dimension: u8,
    /// Specifies if the map that was updated was a locked map, which may be done using a
    /// cartography table.
    pub locked_map: bool,
    /// The center position of the map being updated.
    pub origin: BlockPos,
    /// The scale of the map as it is shown in-game. It is written when any of the map update flags
    /// are set to the update flags field.
    pub scale: u8,
    /// Map IDs that the map updated is included in. This has to do with the scale of the map: Each
    /// map holds its own map ID and all map IDs of maps that include this map and have a bigger
    /// scale. This means that a scale zero map will have five map IDs in this list, whereas a scale
    /// four map will have only one (its own). The actual use of this field remains unknown.
    pub maps_included_in: Vec<v64>,
    /// A list of tracked objects on the map, which may either be entities or blocks. The client
    /// makes sure these tracked objects are actually tracked. (position updated etc.)
    pub tracked_objects: Vec<MapTrackedObject>,
    /// A list of fixed decorations located on the map. The decorations will not change client-side,
    /// unless the server updates them.
    pub decorations: Vec<MapDecoration>,
    /// The width of the texture area that was updated. The width may be a subset of the total width
    /// of the map.
    pub width: i32,
    /// The height of the texture area that was updated. The height may be a subset of the total
    /// height of the map.
    pub height: i32,
    /// The X offset in pixels at which the updated texture area starts. From this X, the updated
    /// texture will extend exactly width pixels to the right.
    pub x_offset: i32,
    /// The Y offset in pixels at which the updated texture area starts. From this Y, the updated
    /// texture will extend exactly height pixels up.
    pub y_offset: i32,
    /// A list of pixel colours for the new texture of the map. It is indexed using [y*height + x].
    pub pixels: Vec<VarRGBA>,
}

impl Encode for ClientBoundMapItemData {
    fn encode(&self, w: &mut Writer) {
        self.map_id.encode(w);
        self.update_flags.encode(w);
        self.dimension.encode(w);
        self.locked_map.encode(w);
        self.origin.encode(w);

        if *self.update_flags & MapUpdateFlag::Initialisation.flag() != 0 {
            self.maps_included_in.encode(w);
        }

        if *self.update_flags
            & (MapUpdateFlag::Initialisation.flag()
            | MapUpdateFlag::Decoration.flag()
            | MapUpdateFlag::Texture.flag())
            != 0
        {
            self.scale.encode(w);
        }

        if *self.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            self.tracked_objects.encode(w);
            self.decorations.encode(w);
        }

        if *self.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            self.width.encode(w);
            self.height.encode(w);
            self.x_offset.encode(w);
            self.y_offset.encode(w);
            self.pixels.encode(w);
        }
    }
}

impl Decode<'_> for ClientBoundMapItemData {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut pk = Self {
            map_id: v64::decode(r)?,
            update_flags: w32::decode(r)?,
            dimension: u8::decode(r)?,
            locked_map : bool::decode(r)?,
            origin: BlockPos::decode(r)?,
            ..Default::default()
        };

        if *pk.update_flags & MapUpdateFlag::Initialisation.flag() != 0 {
            pk.maps_included_in = Vec::decode(r)?;
        }

        if *pk.update_flags
            & (MapUpdateFlag::Initialisation.flag()
                | MapUpdateFlag::Decoration.flag()
                | MapUpdateFlag::Texture.flag())
            != 0
        {
            pk.scale = u8::decode(r)?;
        }
        
        if *pk.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            pk.tracked_objects = Vec::decode(r)?;
            pk.decorations = Vec::decode(r)?;
        }
        
        if *pk.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            pk.width = i32::decode(r)?;
            pk.height = i32::decode(r)?;
            pk.x_offset = i32::decode(r)?;
            pk.y_offset = i32::decode(r)?;
            pk.pixels = Vec::decode(r)?;
        }

        Some(pk)
    }
}
