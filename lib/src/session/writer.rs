use std::cmp::Eq;
use std::hash::Hash;

use image::{
    DynamicImage,
    ImageFormat
};

use crate::swizzler::{
    to_luma_dyn,
    to_lumaa_dyn,
    to_rgb_dyn,
    to_rgba_dyn,
    ChannelDescriptor
};
use crate::errors::{
    ErrorKind
};
use crate::session::{
    Asset,
    GenericAsset
};

/// Generalized texture target.
///
/// Describes how to generate the final image, from a given [`Asset`].
pub trait Target<A: Asset> {

    type Identifier: Hash + Eq;

    /// Generates the texture by swizzling channels of inputs found in the
    /// ```asset``` container.
    fn generate(&self, asset: &A) -> Result<DynamicImage, ErrorKind>;

    /// Returns the file name the generated texture should have.
    fn get_filename(&self, asset: &A) -> String;

    fn get_format(&self) -> image::ImageFormat;

}

/// Generic implementation of the [`Target`] trait.
///
/// This allows to create target at runtime, from the CLI, a JSON, ...
pub struct GenericTarget<Identifier: Eq + Hash + Sync = String> {

    pub name: Option<String>,

    /// Format to use when encoding the texture.
    pub output_format: image::ImageFormat,

    /// Swizzling inputs.
    pub inputs: Vec<Option<(Identifier, u8)>>

}

impl<I: Eq + Hash + Sync> GenericTarget<I> {

    pub fn new(inputs: Vec<Option<(I, u8)>>) -> GenericTarget<I> {
        GenericTarget {
            name: None,
            output_format: image::ImageFormat::PNG,
            inputs
        }
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn _create_descriptor(
        &self,
        index: usize,
        asset: &GenericAsset<I>
    ) -> Result<Option<ChannelDescriptor>, ErrorKind> {
        if let Some(input) = &self.inputs[index] {
            match asset.get_texture_path(&input.0) {
                Some(path) => Ok(Some(ChannelDescriptor::from_path(path, input.1)?)),
                _ => Ok(None)
            }
        } else {
            Ok(None)
        }
    }

}

impl<'a, I: Hash + Eq + Sync + 'a> Target<GenericAsset<'a, I>> for GenericTarget<I> {

    type Identifier = I;

    fn generate(&self, asset: &GenericAsset<'a, I>) -> Result<DynamicImage, ErrorKind> {
        match self.inputs.len() {
            1 => {
                to_luma_dyn(&self._create_descriptor(0, asset)?)
            },
            2 => {
                to_lumaa_dyn(
                    &self._create_descriptor(0, asset)?,
                    &self._create_descriptor(1, asset)?
                )
            },
            3 => {
                to_rgb_dyn(
                    &self._create_descriptor(0, asset)?,
                    &self._create_descriptor(1, asset)?,
                    &self._create_descriptor(2, asset)?,
                )
            },
            a if a >= 4 => {
                to_rgba_dyn(
                    &self._create_descriptor(0, asset)?,
                    &self._create_descriptor(1, asset)?,
                    &self._create_descriptor(2, asset)?,
                    &self._create_descriptor(3, asset)?
                )
            },
            _ => panic!("too big vector!")
        }
    }

    fn get_filename(&self, asset: &GenericAsset<'a, I>) -> String {
        let mut filename = String::from(asset.get_base());
        if let Some(name) = &self.name {
            filename.push_str(name);
        }
        filename
    }

    fn get_format(&self) -> image::ImageFormat {
        self.output_format
    }

}
