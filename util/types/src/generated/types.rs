use molecule::faster_hex::hex_string;
use molecule::prelude::{Entity as _, Reader as _};
#[derive(Clone)]
pub struct BoolOpt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BoolOptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for BoolOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BoolOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for BoolOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for BoolOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct BoolOptBuilder(pub(crate) Option<Bool>);
impl molecule::prelude::Entity for BoolOpt {
    type Builder = BoolOptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        BoolOpt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BoolOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
impl ::std::default::Default for BoolOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        BoolOpt::new_unchecked(v.into())
    }
}
impl BoolOpt {
    pub const NAME: &'static str = "BoolOpt";
    pub fn as_reader(&self) -> BoolOptReader<'_> {
        BoolOptReader::new_unchecked(self.as_slice())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<Bool> {
        if self.is_none() {
            None
        } else {
            Some(Bool::new_unchecked(self.0.clone()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BoolOptReader<'r> {
    type Entity = BoolOpt;
    fn to_entity(&self) -> Self::Entity {
        BoolOpt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BoolOptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            BoolReader::verify(&slice[..])?;
        }
        Ok(())
    }
}
impl<'r> BoolOptReader<'r> {
    pub const NAME: &'r str = "BoolOptReader";
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<BoolReader<'_>> {
        if self.is_none() {
            None
        } else {
            Some(BoolReader::new_unchecked(self.as_slice()))
        }
    }
}
impl molecule::prelude::Builder for BoolOptBuilder {
    type Entity = BoolOpt;
    fn expected_length(&self) -> usize {
        if let Some(ref inner) = self.0 {
            inner.as_slice().len()
        } else {
            0
        }
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        if let Some(ref inner) = self.0 {
            writer.write_all(inner.as_slice())
        } else {
            Ok(())
        }
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        BoolOpt::new_unchecked(inner.into())
    }
}
impl BoolOptBuilder {
    pub const NAME: &'static str = "BoolOptBuilder";
    pub fn set(mut self, v: Option<Bool>) -> Self {
        self.0 = v;
        self
    }
}
#[derive(Clone)]
pub struct Byte32Opt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct Byte32OptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Byte32Opt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for Byte32OptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Byte32Opt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for Byte32OptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct Byte32OptBuilder(pub(crate) Option<Byte32>);
impl molecule::prelude::Entity for Byte32Opt {
    type Builder = Byte32OptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Byte32Opt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Byte32OptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
impl ::std::default::Default for Byte32Opt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        Byte32Opt::new_unchecked(v.into())
    }
}
impl Byte32Opt {
    pub const NAME: &'static str = "Byte32Opt";
    pub fn as_reader(&self) -> Byte32OptReader<'_> {
        Byte32OptReader::new_unchecked(self.as_slice())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<Byte32> {
        if self.is_none() {
            None
        } else {
            Some(Byte32::new_unchecked(self.0.clone()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for Byte32OptReader<'r> {
    type Entity = Byte32Opt;
    fn to_entity(&self) -> Self::Entity {
        Byte32Opt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        Byte32OptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            Byte32Reader::verify(&slice[..])?;
        }
        Ok(())
    }
}
impl<'r> Byte32OptReader<'r> {
    pub const NAME: &'r str = "Byte32OptReader";
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<Byte32Reader<'_>> {
        if self.is_none() {
            None
        } else {
            Some(Byte32Reader::new_unchecked(self.as_slice()))
        }
    }
}
impl molecule::prelude::Builder for Byte32OptBuilder {
    type Entity = Byte32Opt;
    fn expected_length(&self) -> usize {
        if let Some(ref inner) = self.0 {
            inner.as_slice().len()
        } else {
            0
        }
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        if let Some(ref inner) = self.0 {
            writer.write_all(inner.as_slice())
        } else {
            Ok(())
        }
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Byte32Opt::new_unchecked(inner.into())
    }
}
impl Byte32OptBuilder {
    pub const NAME: &'static str = "Byte32OptBuilder";
    pub fn set(mut self, v: Option<Byte32>) -> Self {
        self.0 = v;
        self
    }
}
#[derive(Clone)]
pub struct BytesOpt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BytesOptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for BytesOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BytesOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for BytesOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for BytesOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct BytesOptBuilder(pub(crate) Option<Bytes>);
impl molecule::prelude::Entity for BytesOpt {
    type Builder = BytesOptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        BytesOpt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BytesOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
impl ::std::default::Default for BytesOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        BytesOpt::new_unchecked(v.into())
    }
}
impl BytesOpt {
    pub const NAME: &'static str = "BytesOpt";
    pub fn as_reader(&self) -> BytesOptReader<'_> {
        BytesOptReader::new_unchecked(self.as_slice())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<Bytes> {
        if self.is_none() {
            None
        } else {
            Some(Bytes::new_unchecked(self.0.clone()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BytesOptReader<'r> {
    type Entity = BytesOpt;
    fn to_entity(&self) -> Self::Entity {
        BytesOpt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BytesOptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            BytesReader::verify(&slice[..])?;
        }
        Ok(())
    }
}
impl<'r> BytesOptReader<'r> {
    pub const NAME: &'r str = "BytesOptReader";
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<BytesReader<'_>> {
        if self.is_none() {
            None
        } else {
            Some(BytesReader::new_unchecked(self.as_slice()))
        }
    }
}
impl molecule::prelude::Builder for BytesOptBuilder {
    type Entity = BytesOpt;
    fn expected_length(&self) -> usize {
        if let Some(ref inner) = self.0 {
            inner.as_slice().len()
        } else {
            0
        }
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        if let Some(ref inner) = self.0 {
            writer.write_all(inner.as_slice())
        } else {
            Ok(())
        }
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        BytesOpt::new_unchecked(inner.into())
    }
}
impl BytesOptBuilder {
    pub const NAME: &'static str = "BytesOptBuilder";
    pub fn set(mut self, v: Option<Bytes>) -> Self {
        self.0 = v;
        self
    }
}
#[derive(Clone)]
pub struct Bool(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BoolReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Bool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BoolReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Bool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Display for BoolReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
pub struct BoolBuilder(pub(crate) [u8; 1]);
impl ::std::fmt::Debug for BoolBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}({:?})", Self::NAME, &self.0[..])
    }
}
impl ::std::default::Default for BoolBuilder {
    fn default() -> Self {
        BoolBuilder([0; 1])
    }
}
impl molecule::prelude::Entity for Bool {
    type Builder = BoolBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Bool(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BoolReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set([self.nth0()])
    }
}
impl ::std::default::Default for Bool {
    fn default() -> Self {
        let v: Vec<u8> = vec![0];
        Bool::new_unchecked(v.into())
    }
}
impl Bool {
    pub const NAME: &'static str = "Bool";
    pub fn as_reader(&self) -> BoolReader<'_> {
        BoolReader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 1;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 1;
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.as_bytes()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
}
impl<'r> molecule::prelude::Reader<'r> for BoolReader<'r> {
    type Entity = Bool;
    fn to_entity(&self) -> Self::Entity {
        Bool::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BoolReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 1 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 1, slice.len());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> BoolReader<'r> {
    pub const NAME: &'r str = "BoolReader";
    pub const TOTAL_SIZE: usize = 1;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 1;
    pub fn raw_data(&self) -> &[u8] {
        self.as_slice()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
}
impl molecule::prelude::Builder for BoolBuilder {
    type Entity = Bool;
    fn expected_length(&self) -> usize {
        1
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Bool::new_unchecked(inner.into())
    }
}
impl BoolBuilder {
    pub const NAME: &'static str = "BoolBuilder";
    pub fn set(mut self, v: [u8; 1]) -> Self {
        self.0 = v;
        self
    }
    pub fn nth0(mut self, v: u8) -> Self {
        self.0[0] = v;
        self
    }
}
#[derive(Clone)]
pub struct Uint32(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct Uint32Reader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Uint32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for Uint32Reader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Uint32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Display for Uint32Reader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
pub struct Uint32Builder(pub(crate) [u8; 4]);
impl ::std::fmt::Debug for Uint32Builder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}({:?})", Self::NAME, &self.0[..])
    }
}
impl ::std::default::Default for Uint32Builder {
    fn default() -> Self {
        Uint32Builder([0; 4])
    }
}
impl molecule::prelude::Entity for Uint32 {
    type Builder = Uint32Builder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Uint32(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Uint32Reader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set([self.nth0(), self.nth1(), self.nth2(), self.nth3()])
    }
}
impl ::std::default::Default for Uint32 {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        Uint32::new_unchecked(v.into())
    }
}
impl Uint32 {
    pub const NAME: &'static str = "Uint32";
    pub fn as_reader(&self) -> Uint32Reader<'_> {
        Uint32Reader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 4;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 4;
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.as_bytes()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
}
impl<'r> molecule::prelude::Reader<'r> for Uint32Reader<'r> {
    type Entity = Uint32;
    fn to_entity(&self) -> Self::Entity {
        Uint32::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        Uint32Reader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 4 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 4, slice.len());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> Uint32Reader<'r> {
    pub const NAME: &'r str = "Uint32Reader";
    pub const TOTAL_SIZE: usize = 4;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 4;
    pub fn raw_data(&self) -> &[u8] {
        self.as_slice()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
}
impl molecule::prelude::Builder for Uint32Builder {
    type Entity = Uint32;
    fn expected_length(&self) -> usize {
        4
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Uint32::new_unchecked(inner.into())
    }
}
impl Uint32Builder {
    pub const NAME: &'static str = "Uint32Builder";
    pub fn set(mut self, v: [u8; 4]) -> Self {
        self.0 = v;
        self
    }
    pub fn nth0(mut self, v: u8) -> Self {
        self.0[0] = v;
        self
    }
    pub fn nth1(mut self, v: u8) -> Self {
        self.0[1] = v;
        self
    }
    pub fn nth2(mut self, v: u8) -> Self {
        self.0[2] = v;
        self
    }
    pub fn nth3(mut self, v: u8) -> Self {
        self.0[3] = v;
        self
    }
}
#[derive(Clone)]
pub struct Uint64(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct Uint64Reader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Uint64 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for Uint64Reader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Uint64 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Display for Uint64Reader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
pub struct Uint64Builder(pub(crate) [u8; 8]);
impl ::std::fmt::Debug for Uint64Builder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}({:?})", Self::NAME, &self.0[..])
    }
}
impl ::std::default::Default for Uint64Builder {
    fn default() -> Self {
        Uint64Builder([0; 8])
    }
}
impl molecule::prelude::Entity for Uint64 {
    type Builder = Uint64Builder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Uint64(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Uint64Reader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set([
            self.nth0(),
            self.nth1(),
            self.nth2(),
            self.nth3(),
            self.nth4(),
            self.nth5(),
            self.nth6(),
            self.nth7(),
        ])
    }
}
impl ::std::default::Default for Uint64 {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        Uint64::new_unchecked(v.into())
    }
}
impl Uint64 {
    pub const NAME: &'static str = "Uint64";
    pub fn as_reader(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 8;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 8;
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.as_bytes()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
    pub fn nth4(&self) -> u8 {
        self.0[4]
    }
    pub fn nth5(&self) -> u8 {
        self.0[5]
    }
    pub fn nth6(&self) -> u8 {
        self.0[6]
    }
    pub fn nth7(&self) -> u8 {
        self.0[7]
    }
}
impl<'r> molecule::prelude::Reader<'r> for Uint64Reader<'r> {
    type Entity = Uint64;
    fn to_entity(&self) -> Self::Entity {
        Uint64::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        Uint64Reader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 8 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 8, slice.len());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> Uint64Reader<'r> {
    pub const NAME: &'r str = "Uint64Reader";
    pub const TOTAL_SIZE: usize = 8;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 8;
    pub fn raw_data(&self) -> &[u8] {
        self.as_slice()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
    pub fn nth4(&self) -> u8 {
        self.0[4]
    }
    pub fn nth5(&self) -> u8 {
        self.0[5]
    }
    pub fn nth6(&self) -> u8 {
        self.0[6]
    }
    pub fn nth7(&self) -> u8 {
        self.0[7]
    }
}
impl molecule::prelude::Builder for Uint64Builder {
    type Entity = Uint64;
    fn expected_length(&self) -> usize {
        8
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Uint64::new_unchecked(inner.into())
    }
}
impl Uint64Builder {
    pub const NAME: &'static str = "Uint64Builder";
    pub fn set(mut self, v: [u8; 8]) -> Self {
        self.0 = v;
        self
    }
    pub fn nth0(mut self, v: u8) -> Self {
        self.0[0] = v;
        self
    }
    pub fn nth1(mut self, v: u8) -> Self {
        self.0[1] = v;
        self
    }
    pub fn nth2(mut self, v: u8) -> Self {
        self.0[2] = v;
        self
    }
    pub fn nth3(mut self, v: u8) -> Self {
        self.0[3] = v;
        self
    }
    pub fn nth4(mut self, v: u8) -> Self {
        self.0[4] = v;
        self
    }
    pub fn nth5(mut self, v: u8) -> Self {
        self.0[5] = v;
        self
    }
    pub fn nth6(mut self, v: u8) -> Self {
        self.0[6] = v;
        self
    }
    pub fn nth7(mut self, v: u8) -> Self {
        self.0[7] = v;
        self
    }
}
#[derive(Clone)]
pub struct Byte32(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct Byte32Reader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Byte32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for Byte32Reader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Byte32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Display for Byte32Reader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
pub struct Byte32Builder(pub(crate) [u8; 32]);
impl ::std::fmt::Debug for Byte32Builder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}({:?})", Self::NAME, &self.0[..])
    }
}
impl ::std::default::Default for Byte32Builder {
    fn default() -> Self {
        Byte32Builder([0; 32])
    }
}
impl molecule::prelude::Entity for Byte32 {
    type Builder = Byte32Builder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Byte32(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Byte32Reader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set([
            self.nth0(),
            self.nth1(),
            self.nth2(),
            self.nth3(),
            self.nth4(),
            self.nth5(),
            self.nth6(),
            self.nth7(),
            self.nth8(),
            self.nth9(),
            self.nth10(),
            self.nth11(),
            self.nth12(),
            self.nth13(),
            self.nth14(),
            self.nth15(),
            self.nth16(),
            self.nth17(),
            self.nth18(),
            self.nth19(),
            self.nth20(),
            self.nth21(),
            self.nth22(),
            self.nth23(),
            self.nth24(),
            self.nth25(),
            self.nth26(),
            self.nth27(),
            self.nth28(),
            self.nth29(),
            self.nth30(),
            self.nth31(),
        ])
    }
}
impl ::std::default::Default for Byte32 {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        Byte32::new_unchecked(v.into())
    }
}
impl Byte32 {
    pub const NAME: &'static str = "Byte32";
    pub fn as_reader(&self) -> Byte32Reader<'_> {
        Byte32Reader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 32;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 32;
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.as_bytes()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
    pub fn nth4(&self) -> u8 {
        self.0[4]
    }
    pub fn nth5(&self) -> u8 {
        self.0[5]
    }
    pub fn nth6(&self) -> u8 {
        self.0[6]
    }
    pub fn nth7(&self) -> u8 {
        self.0[7]
    }
    pub fn nth8(&self) -> u8 {
        self.0[8]
    }
    pub fn nth9(&self) -> u8 {
        self.0[9]
    }
    pub fn nth10(&self) -> u8 {
        self.0[10]
    }
    pub fn nth11(&self) -> u8 {
        self.0[11]
    }
    pub fn nth12(&self) -> u8 {
        self.0[12]
    }
    pub fn nth13(&self) -> u8 {
        self.0[13]
    }
    pub fn nth14(&self) -> u8 {
        self.0[14]
    }
    pub fn nth15(&self) -> u8 {
        self.0[15]
    }
    pub fn nth16(&self) -> u8 {
        self.0[16]
    }
    pub fn nth17(&self) -> u8 {
        self.0[17]
    }
    pub fn nth18(&self) -> u8 {
        self.0[18]
    }
    pub fn nth19(&self) -> u8 {
        self.0[19]
    }
    pub fn nth20(&self) -> u8 {
        self.0[20]
    }
    pub fn nth21(&self) -> u8 {
        self.0[21]
    }
    pub fn nth22(&self) -> u8 {
        self.0[22]
    }
    pub fn nth23(&self) -> u8 {
        self.0[23]
    }
    pub fn nth24(&self) -> u8 {
        self.0[24]
    }
    pub fn nth25(&self) -> u8 {
        self.0[25]
    }
    pub fn nth26(&self) -> u8 {
        self.0[26]
    }
    pub fn nth27(&self) -> u8 {
        self.0[27]
    }
    pub fn nth28(&self) -> u8 {
        self.0[28]
    }
    pub fn nth29(&self) -> u8 {
        self.0[29]
    }
    pub fn nth30(&self) -> u8 {
        self.0[30]
    }
    pub fn nth31(&self) -> u8 {
        self.0[31]
    }
}
impl<'r> molecule::prelude::Reader<'r> for Byte32Reader<'r> {
    type Entity = Byte32;
    fn to_entity(&self) -> Self::Entity {
        Byte32::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        Byte32Reader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 32 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 32, slice.len());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> Byte32Reader<'r> {
    pub const NAME: &'r str = "Byte32Reader";
    pub const TOTAL_SIZE: usize = 32;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 32;
    pub fn raw_data(&self) -> &[u8] {
        self.as_slice()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
    pub fn nth4(&self) -> u8 {
        self.0[4]
    }
    pub fn nth5(&self) -> u8 {
        self.0[5]
    }
    pub fn nth6(&self) -> u8 {
        self.0[6]
    }
    pub fn nth7(&self) -> u8 {
        self.0[7]
    }
    pub fn nth8(&self) -> u8 {
        self.0[8]
    }
    pub fn nth9(&self) -> u8 {
        self.0[9]
    }
    pub fn nth10(&self) -> u8 {
        self.0[10]
    }
    pub fn nth11(&self) -> u8 {
        self.0[11]
    }
    pub fn nth12(&self) -> u8 {
        self.0[12]
    }
    pub fn nth13(&self) -> u8 {
        self.0[13]
    }
    pub fn nth14(&self) -> u8 {
        self.0[14]
    }
    pub fn nth15(&self) -> u8 {
        self.0[15]
    }
    pub fn nth16(&self) -> u8 {
        self.0[16]
    }
    pub fn nth17(&self) -> u8 {
        self.0[17]
    }
    pub fn nth18(&self) -> u8 {
        self.0[18]
    }
    pub fn nth19(&self) -> u8 {
        self.0[19]
    }
    pub fn nth20(&self) -> u8 {
        self.0[20]
    }
    pub fn nth21(&self) -> u8 {
        self.0[21]
    }
    pub fn nth22(&self) -> u8 {
        self.0[22]
    }
    pub fn nth23(&self) -> u8 {
        self.0[23]
    }
    pub fn nth24(&self) -> u8 {
        self.0[24]
    }
    pub fn nth25(&self) -> u8 {
        self.0[25]
    }
    pub fn nth26(&self) -> u8 {
        self.0[26]
    }
    pub fn nth27(&self) -> u8 {
        self.0[27]
    }
    pub fn nth28(&self) -> u8 {
        self.0[28]
    }
    pub fn nth29(&self) -> u8 {
        self.0[29]
    }
    pub fn nth30(&self) -> u8 {
        self.0[30]
    }
    pub fn nth31(&self) -> u8 {
        self.0[31]
    }
}
impl molecule::prelude::Builder for Byte32Builder {
    type Entity = Byte32;
    fn expected_length(&self) -> usize {
        32
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Byte32::new_unchecked(inner.into())
    }
}
impl Byte32Builder {
    pub const NAME: &'static str = "Byte32Builder";
    pub fn set(mut self, v: [u8; 32]) -> Self {
        self.0 = v;
        self
    }
    pub fn nth0(mut self, v: u8) -> Self {
        self.0[0] = v;
        self
    }
    pub fn nth1(mut self, v: u8) -> Self {
        self.0[1] = v;
        self
    }
    pub fn nth2(mut self, v: u8) -> Self {
        self.0[2] = v;
        self
    }
    pub fn nth3(mut self, v: u8) -> Self {
        self.0[3] = v;
        self
    }
    pub fn nth4(mut self, v: u8) -> Self {
        self.0[4] = v;
        self
    }
    pub fn nth5(mut self, v: u8) -> Self {
        self.0[5] = v;
        self
    }
    pub fn nth6(mut self, v: u8) -> Self {
        self.0[6] = v;
        self
    }
    pub fn nth7(mut self, v: u8) -> Self {
        self.0[7] = v;
        self
    }
    pub fn nth8(mut self, v: u8) -> Self {
        self.0[8] = v;
        self
    }
    pub fn nth9(mut self, v: u8) -> Self {
        self.0[9] = v;
        self
    }
    pub fn nth10(mut self, v: u8) -> Self {
        self.0[10] = v;
        self
    }
    pub fn nth11(mut self, v: u8) -> Self {
        self.0[11] = v;
        self
    }
    pub fn nth12(mut self, v: u8) -> Self {
        self.0[12] = v;
        self
    }
    pub fn nth13(mut self, v: u8) -> Self {
        self.0[13] = v;
        self
    }
    pub fn nth14(mut self, v: u8) -> Self {
        self.0[14] = v;
        self
    }
    pub fn nth15(mut self, v: u8) -> Self {
        self.0[15] = v;
        self
    }
    pub fn nth16(mut self, v: u8) -> Self {
        self.0[16] = v;
        self
    }
    pub fn nth17(mut self, v: u8) -> Self {
        self.0[17] = v;
        self
    }
    pub fn nth18(mut self, v: u8) -> Self {
        self.0[18] = v;
        self
    }
    pub fn nth19(mut self, v: u8) -> Self {
        self.0[19] = v;
        self
    }
    pub fn nth20(mut self, v: u8) -> Self {
        self.0[20] = v;
        self
    }
    pub fn nth21(mut self, v: u8) -> Self {
        self.0[21] = v;
        self
    }
    pub fn nth22(mut self, v: u8) -> Self {
        self.0[22] = v;
        self
    }
    pub fn nth23(mut self, v: u8) -> Self {
        self.0[23] = v;
        self
    }
    pub fn nth24(mut self, v: u8) -> Self {
        self.0[24] = v;
        self
    }
    pub fn nth25(mut self, v: u8) -> Self {
        self.0[25] = v;
        self
    }
    pub fn nth26(mut self, v: u8) -> Self {
        self.0[26] = v;
        self
    }
    pub fn nth27(mut self, v: u8) -> Self {
        self.0[27] = v;
        self
    }
    pub fn nth28(mut self, v: u8) -> Self {
        self.0[28] = v;
        self
    }
    pub fn nth29(mut self, v: u8) -> Self {
        self.0[29] = v;
        self
    }
    pub fn nth30(mut self, v: u8) -> Self {
        self.0[30] = v;
        self
    }
    pub fn nth31(mut self, v: u8) -> Self {
        self.0[31] = v;
        self
    }
}
#[derive(Clone)]
pub struct Bytes(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BytesReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BytesReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Display for BytesReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
#[derive(Debug, Default)]
pub struct BytesBuilder(pub(crate) Vec<u8>);
impl molecule::prelude::Entity for Bytes {
    type Builder = BytesBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Bytes(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BytesReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for Bytes {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        Bytes::new_unchecked(v.into())
    }
}
impl Bytes {
    pub const NAME: &'static str = "Bytes";
    pub fn as_reader(&self) -> BytesReader<'_> {
        BytesReader::new_unchecked(self.as_slice())
    }
    pub const ITEM_SIZE: usize = 1;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.0.slice_from(4)
    }
    pub fn get(&self, idx: usize) -> Option<u8> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> u8 {
        self.0[4 + idx]
    }
}
impl<'r> molecule::prelude::Reader<'r> for BytesReader<'r> {
    type Entity = Bytes;
    fn to_entity(&self) -> Self::Entity {
        Bytes::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BytesReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let item_count = u32::from_le(ptr[0]) as usize;
        let expected = 4 + 1 * item_count;
        if len != expected {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), expected, len);
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> BytesReader<'r> {
    pub const NAME: &'r str = "BytesReader";
    pub const ITEM_SIZE: usize = 1;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn raw_data(&self) -> &[u8] {
        &self.as_slice()[4..]
    }
    pub fn get(&self, idx: usize) -> Option<u8> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> u8 {
        self.0[4 + idx]
    }
}
impl molecule::prelude::Builder for BytesBuilder {
    type Entity = Bytes;
    fn expected_length(&self) -> usize {
        4 + 1 * self.0.len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.0.len() as u32).to_le_bytes();
        writer.write_all(&len)?;
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Bytes::new_unchecked(inner.into())
    }
}
impl BytesBuilder {
    pub const NAME: &'static str = "BytesBuilder";
    pub fn set(mut self, v: Vec<u8>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: u8) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = u8>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct BytesIterator(Bytes, usize, usize);
impl ::std::iter::Iterator for BytesIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for BytesIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = BytesIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        BytesIterator(self, 0, len)
    }
}
#[derive(Clone)]
pub struct Uint32Vec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct Uint32VecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Uint32Vec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for Uint32VecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Uint32Vec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for Uint32VecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct Uint32VecBuilder(pub(crate) Vec<Uint32>);
impl molecule::prelude::Entity for Uint32Vec {
    type Builder = Uint32VecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Uint32Vec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Uint32VecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for Uint32Vec {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        Uint32Vec::new_unchecked(v.into())
    }
}
impl Uint32Vec {
    pub const NAME: &'static str = "Uint32Vec";
    pub fn as_reader(&self) -> Uint32VecReader<'_> {
        Uint32VecReader::new_unchecked(self.as_slice())
    }
    pub const ITEM_SIZE: usize = 4;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Uint32> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Uint32 {
        let start = 4 + idx * 4;
        let end = start + 4;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
}
impl<'r> molecule::prelude::Reader<'r> for Uint32VecReader<'r> {
    type Entity = Uint32Vec;
    fn to_entity(&self) -> Self::Entity {
        Uint32Vec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        Uint32VecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let item_count = u32::from_le(ptr[0]) as usize;
        let expected = 4 + 4 * item_count;
        if len != expected {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), expected, len);
            Err(err)?;
        }
        for i in 0..item_count {
            let start = 4 * i;
            let end = start + 4;
            Uint32Reader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> Uint32VecReader<'r> {
    pub const NAME: &'r str = "Uint32VecReader";
    pub const ITEM_SIZE: usize = 4;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Uint32Reader<'_>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Uint32Reader<'_> {
        let start = 4 + idx * 4;
        let end = start + 4;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl molecule::prelude::Builder for Uint32VecBuilder {
    type Entity = Uint32Vec;
    fn expected_length(&self) -> usize {
        4 + 4 * self.0.len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.0.len() as u32).to_le_bytes();
        writer.write_all(&len)?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Uint32Vec::new_unchecked(inner.into())
    }
}
impl Uint32VecBuilder {
    pub const NAME: &'static str = "Uint32VecBuilder";
    pub fn set(mut self, v: Vec<Uint32>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Uint32) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Uint32>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct Uint32VecIterator(Uint32Vec, usize, usize);
impl ::std::iter::Iterator for Uint32VecIterator {
    type Item = Uint32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for Uint32VecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for Uint32Vec {
    type Item = Uint32;
    type IntoIter = Uint32VecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        Uint32VecIterator(self, 0, len)
    }
}
impl<'r> Uint32VecReader<'r> {
    pub fn iter(&self) -> Uint32VecReaderIterator<'_, 'r> {
        Uint32VecReaderIterator(&self, 0, self.len())
    }
}
pub struct Uint32VecReaderIterator<'t, 'r>(&'t Uint32VecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for Uint32VecReaderIterator<'t, 'r> {
    type Item = Uint32Reader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for Uint32VecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct Uint64Vec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct Uint64VecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Uint64Vec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for Uint64VecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Uint64Vec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for Uint64VecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct Uint64VecBuilder(pub(crate) Vec<Uint64>);
impl molecule::prelude::Entity for Uint64Vec {
    type Builder = Uint64VecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Uint64Vec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Uint64VecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for Uint64Vec {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        Uint64Vec::new_unchecked(v.into())
    }
}
impl Uint64Vec {
    pub const NAME: &'static str = "Uint64Vec";
    pub fn as_reader(&self) -> Uint64VecReader<'_> {
        Uint64VecReader::new_unchecked(self.as_slice())
    }
    pub const ITEM_SIZE: usize = 8;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Uint64> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Uint64 {
        let start = 4 + idx * 8;
        let end = start + 8;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
}
impl<'r> molecule::prelude::Reader<'r> for Uint64VecReader<'r> {
    type Entity = Uint64Vec;
    fn to_entity(&self) -> Self::Entity {
        Uint64Vec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        Uint64VecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let item_count = u32::from_le(ptr[0]) as usize;
        let expected = 4 + 8 * item_count;
        if len != expected {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), expected, len);
            Err(err)?;
        }
        for i in 0..item_count {
            let start = 8 * i;
            let end = start + 8;
            Uint64Reader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> Uint64VecReader<'r> {
    pub const NAME: &'r str = "Uint64VecReader";
    pub const ITEM_SIZE: usize = 8;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Uint64Reader<'_>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Uint64Reader<'_> {
        let start = 4 + idx * 8;
        let end = start + 8;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl molecule::prelude::Builder for Uint64VecBuilder {
    type Entity = Uint64Vec;
    fn expected_length(&self) -> usize {
        4 + 8 * self.0.len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.0.len() as u32).to_le_bytes();
        writer.write_all(&len)?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Uint64Vec::new_unchecked(inner.into())
    }
}
impl Uint64VecBuilder {
    pub const NAME: &'static str = "Uint64VecBuilder";
    pub fn set(mut self, v: Vec<Uint64>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Uint64) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Uint64>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct Uint64VecIterator(Uint64Vec, usize, usize);
impl ::std::iter::Iterator for Uint64VecIterator {
    type Item = Uint64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for Uint64VecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for Uint64Vec {
    type Item = Uint64;
    type IntoIter = Uint64VecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        Uint64VecIterator(self, 0, len)
    }
}
impl<'r> Uint64VecReader<'r> {
    pub fn iter(&self) -> Uint64VecReaderIterator<'_, 'r> {
        Uint64VecReaderIterator(&self, 0, self.len())
    }
}
pub struct Uint64VecReaderIterator<'t, 'r>(&'t Uint64VecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for Uint64VecReaderIterator<'t, 'r> {
    type Item = Uint64Reader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for Uint64VecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct BytesVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BytesVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for BytesVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BytesVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for BytesVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for BytesVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct BytesVecBuilder(pub(crate) Vec<Bytes>);
impl molecule::prelude::Entity for BytesVec {
    type Builder = BytesVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        BytesVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BytesVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for BytesVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        BytesVec::new_unchecked(v.into())
    }
}
impl BytesVec {
    pub const NAME: &'static str = "BytesVec";
    pub fn as_reader(&self) -> BytesVecReader<'_> {
        BytesVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Bytes> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Bytes {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BytesVecReader<'r> {
    type Entity = BytesVec;
    fn to_entity(&self) -> Self::Entity {
        BytesVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BytesVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            BytesReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> BytesVecReader<'r> {
    pub const NAME: &'r str = "BytesVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<BytesReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> BytesReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for BytesVecBuilder {
    type Entity = BytesVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        BytesVec::new_unchecked(inner.into())
    }
}
impl BytesVecBuilder {
    pub const NAME: &'static str = "BytesVecBuilder";
    pub fn set(mut self, v: Vec<Bytes>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Bytes) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Bytes>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct BytesVecIterator(BytesVec, usize, usize);
impl ::std::iter::Iterator for BytesVecIterator {
    type Item = Bytes;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for BytesVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for BytesVec {
    type Item = Bytes;
    type IntoIter = BytesVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        BytesVecIterator(self, 0, len)
    }
}
impl<'r> BytesVecReader<'r> {
    pub fn iter(&self) -> BytesVecReaderIterator<'_, 'r> {
        BytesVecReaderIterator(&self, 0, self.len())
    }
}
pub struct BytesVecReaderIterator<'t, 'r>(&'t BytesVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for BytesVecReaderIterator<'t, 'r> {
    type Item = BytesReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for BytesVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct Byte32Vec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct Byte32VecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Byte32Vec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for Byte32VecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Byte32Vec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for Byte32VecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct Byte32VecBuilder(pub(crate) Vec<Byte32>);
impl molecule::prelude::Entity for Byte32Vec {
    type Builder = Byte32VecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Byte32Vec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Byte32VecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for Byte32Vec {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        Byte32Vec::new_unchecked(v.into())
    }
}
impl Byte32Vec {
    pub const NAME: &'static str = "Byte32Vec";
    pub fn as_reader(&self) -> Byte32VecReader<'_> {
        Byte32VecReader::new_unchecked(self.as_slice())
    }
    pub const ITEM_SIZE: usize = 32;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Byte32> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Byte32 {
        let start = 4 + idx * 32;
        let end = start + 32;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
}
impl<'r> molecule::prelude::Reader<'r> for Byte32VecReader<'r> {
    type Entity = Byte32Vec;
    fn to_entity(&self) -> Self::Entity {
        Byte32Vec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        Byte32VecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let item_count = u32::from_le(ptr[0]) as usize;
        let expected = 4 + 32 * item_count;
        if len != expected {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), expected, len);
            Err(err)?;
        }
        for i in 0..item_count {
            let start = 32 * i;
            let end = start + 32;
            Byte32Reader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> Byte32VecReader<'r> {
    pub const NAME: &'r str = "Byte32VecReader";
    pub const ITEM_SIZE: usize = 32;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Byte32Reader<'_>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Byte32Reader<'_> {
        let start = 4 + idx * 32;
        let end = start + 32;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl molecule::prelude::Builder for Byte32VecBuilder {
    type Entity = Byte32Vec;
    fn expected_length(&self) -> usize {
        4 + 32 * self.0.len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.0.len() as u32).to_le_bytes();
        writer.write_all(&len)?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Byte32Vec::new_unchecked(inner.into())
    }
}
impl Byte32VecBuilder {
    pub const NAME: &'static str = "Byte32VecBuilder";
    pub fn set(mut self, v: Vec<Byte32>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Byte32) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Byte32>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct Byte32VecIterator(Byte32Vec, usize, usize);
impl ::std::iter::Iterator for Byte32VecIterator {
    type Item = Byte32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for Byte32VecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for Byte32Vec {
    type Item = Byte32;
    type IntoIter = Byte32VecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        Byte32VecIterator(self, 0, len)
    }
}
impl<'r> Byte32VecReader<'r> {
    pub fn iter(&self) -> Byte32VecReaderIterator<'_, 'r> {
        Byte32VecReaderIterator(&self, 0, self.len())
    }
}
pub struct Byte32VecReaderIterator<'t, 'r>(&'t Byte32VecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for Byte32VecReaderIterator<'t, 'r> {
    type Item = Byte32Reader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for Byte32VecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct CellOutputOpt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellOutputOptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellOutputOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellOutputOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellOutputOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for CellOutputOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct CellOutputOptBuilder(pub(crate) Option<CellOutput>);
impl molecule::prelude::Entity for CellOutputOpt {
    type Builder = CellOutputOptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellOutputOpt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellOutputOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
impl ::std::default::Default for CellOutputOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        CellOutputOpt::new_unchecked(v.into())
    }
}
impl CellOutputOpt {
    pub const NAME: &'static str = "CellOutputOpt";
    pub fn as_reader(&self) -> CellOutputOptReader<'_> {
        CellOutputOptReader::new_unchecked(self.as_slice())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<CellOutput> {
        if self.is_none() {
            None
        } else {
            Some(CellOutput::new_unchecked(self.0.clone()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellOutputOptReader<'r> {
    type Entity = CellOutputOpt;
    fn to_entity(&self) -> Self::Entity {
        CellOutputOpt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellOutputOptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            CellOutputReader::verify(&slice[..])?;
        }
        Ok(())
    }
}
impl<'r> CellOutputOptReader<'r> {
    pub const NAME: &'r str = "CellOutputOptReader";
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<CellOutputReader<'_>> {
        if self.is_none() {
            None
        } else {
            Some(CellOutputReader::new_unchecked(self.as_slice()))
        }
    }
}
impl molecule::prelude::Builder for CellOutputOptBuilder {
    type Entity = CellOutputOpt;
    fn expected_length(&self) -> usize {
        if let Some(ref inner) = self.0 {
            inner.as_slice().len()
        } else {
            0
        }
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        if let Some(ref inner) = self.0 {
            writer.write_all(inner.as_slice())
        } else {
            Ok(())
        }
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellOutputOpt::new_unchecked(inner.into())
    }
}
impl CellOutputOptBuilder {
    pub const NAME: &'static str = "CellOutputOptBuilder";
    pub fn set(mut self, v: Option<CellOutput>) -> Self {
        self.0 = v;
        self
    }
}
#[derive(Clone)]
pub struct CellOutPointOpt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellOutPointOptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellOutPointOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellOutPointOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellOutPointOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for CellOutPointOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct CellOutPointOptBuilder(pub(crate) Option<CellOutPoint>);
impl molecule::prelude::Entity for CellOutPointOpt {
    type Builder = CellOutPointOptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellOutPointOpt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellOutPointOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
impl ::std::default::Default for CellOutPointOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        CellOutPointOpt::new_unchecked(v.into())
    }
}
impl CellOutPointOpt {
    pub const NAME: &'static str = "CellOutPointOpt";
    pub fn as_reader(&self) -> CellOutPointOptReader<'_> {
        CellOutPointOptReader::new_unchecked(self.as_slice())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<CellOutPoint> {
        if self.is_none() {
            None
        } else {
            Some(CellOutPoint::new_unchecked(self.0.clone()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellOutPointOptReader<'r> {
    type Entity = CellOutPointOpt;
    fn to_entity(&self) -> Self::Entity {
        CellOutPointOpt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellOutPointOptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            CellOutPointReader::verify(&slice[..])?;
        }
        Ok(())
    }
}
impl<'r> CellOutPointOptReader<'r> {
    pub const NAME: &'r str = "CellOutPointOptReader";
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<CellOutPointReader<'_>> {
        if self.is_none() {
            None
        } else {
            Some(CellOutPointReader::new_unchecked(self.as_slice()))
        }
    }
}
impl molecule::prelude::Builder for CellOutPointOptBuilder {
    type Entity = CellOutPointOpt;
    fn expected_length(&self) -> usize {
        if let Some(ref inner) = self.0 {
            inner.as_slice().len()
        } else {
            0
        }
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        if let Some(ref inner) = self.0 {
            writer.write_all(inner.as_slice())
        } else {
            Ok(())
        }
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellOutPointOpt::new_unchecked(inner.into())
    }
}
impl CellOutPointOptBuilder {
    pub const NAME: &'static str = "CellOutPointOptBuilder";
    pub fn set(mut self, v: Option<CellOutPoint>) -> Self {
        self.0 = v;
        self
    }
}
#[derive(Clone)]
pub struct ScriptOpt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct ScriptOptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for ScriptOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for ScriptOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for ScriptOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for ScriptOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct ScriptOptBuilder(pub(crate) Option<Script>);
impl molecule::prelude::Entity for ScriptOpt {
    type Builder = ScriptOptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ScriptOpt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
impl ::std::default::Default for ScriptOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        ScriptOpt::new_unchecked(v.into())
    }
}
impl ScriptOpt {
    pub const NAME: &'static str = "ScriptOpt";
    pub fn as_reader(&self) -> ScriptOptReader<'_> {
        ScriptOptReader::new_unchecked(self.as_slice())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<Script> {
        if self.is_none() {
            None
        } else {
            Some(Script::new_unchecked(self.0.clone()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ScriptOptReader<'r> {
    type Entity = ScriptOpt;
    fn to_entity(&self) -> Self::Entity {
        ScriptOpt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ScriptOptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            ScriptReader::verify(&slice[..])?;
        }
        Ok(())
    }
}
impl<'r> ScriptOptReader<'r> {
    pub const NAME: &'r str = "ScriptOptReader";
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<ScriptReader<'_>> {
        if self.is_none() {
            None
        } else {
            Some(ScriptReader::new_unchecked(self.as_slice()))
        }
    }
}
impl molecule::prelude::Builder for ScriptOptBuilder {
    type Entity = ScriptOpt;
    fn expected_length(&self) -> usize {
        if let Some(ref inner) = self.0 {
            inner.as_slice().len()
        } else {
            0
        }
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        if let Some(ref inner) = self.0 {
            writer.write_all(inner.as_slice())
        } else {
            Ok(())
        }
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        ScriptOpt::new_unchecked(inner.into())
    }
}
impl ScriptOptBuilder {
    pub const NAME: &'static str = "ScriptOptBuilder";
    pub fn set(mut self, v: Option<Script>) -> Self {
        self.0 = v;
        self
    }
}
#[derive(Clone)]
pub struct ProposalShortId(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct ProposalShortIdReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for ProposalShortId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for ProposalShortIdReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for ProposalShortId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Display for ProposalShortIdReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
pub struct ProposalShortIdBuilder(pub(crate) [u8; 10]);
impl ::std::fmt::Debug for ProposalShortIdBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}({:?})", Self::NAME, &self.0[..])
    }
}
impl ::std::default::Default for ProposalShortIdBuilder {
    fn default() -> Self {
        ProposalShortIdBuilder([0; 10])
    }
}
impl molecule::prelude::Entity for ProposalShortId {
    type Builder = ProposalShortIdBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ProposalShortId(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ProposalShortIdReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set([
            self.nth0(),
            self.nth1(),
            self.nth2(),
            self.nth3(),
            self.nth4(),
            self.nth5(),
            self.nth6(),
            self.nth7(),
            self.nth8(),
            self.nth9(),
        ])
    }
}
impl ::std::default::Default for ProposalShortId {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        ProposalShortId::new_unchecked(v.into())
    }
}
impl ProposalShortId {
    pub const NAME: &'static str = "ProposalShortId";
    pub fn as_reader(&self) -> ProposalShortIdReader<'_> {
        ProposalShortIdReader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 10;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 10;
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.as_bytes()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
    pub fn nth4(&self) -> u8 {
        self.0[4]
    }
    pub fn nth5(&self) -> u8 {
        self.0[5]
    }
    pub fn nth6(&self) -> u8 {
        self.0[6]
    }
    pub fn nth7(&self) -> u8 {
        self.0[7]
    }
    pub fn nth8(&self) -> u8 {
        self.0[8]
    }
    pub fn nth9(&self) -> u8 {
        self.0[9]
    }
}
impl<'r> molecule::prelude::Reader<'r> for ProposalShortIdReader<'r> {
    type Entity = ProposalShortId;
    fn to_entity(&self) -> Self::Entity {
        ProposalShortId::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ProposalShortIdReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 10 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 10, slice.len());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> ProposalShortIdReader<'r> {
    pub const NAME: &'r str = "ProposalShortIdReader";
    pub const TOTAL_SIZE: usize = 10;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 10;
    pub fn raw_data(&self) -> &[u8] {
        self.as_slice()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
    pub fn nth1(&self) -> u8 {
        self.0[1]
    }
    pub fn nth2(&self) -> u8 {
        self.0[2]
    }
    pub fn nth3(&self) -> u8 {
        self.0[3]
    }
    pub fn nth4(&self) -> u8 {
        self.0[4]
    }
    pub fn nth5(&self) -> u8 {
        self.0[5]
    }
    pub fn nth6(&self) -> u8 {
        self.0[6]
    }
    pub fn nth7(&self) -> u8 {
        self.0[7]
    }
    pub fn nth8(&self) -> u8 {
        self.0[8]
    }
    pub fn nth9(&self) -> u8 {
        self.0[9]
    }
}
impl molecule::prelude::Builder for ProposalShortIdBuilder {
    type Entity = ProposalShortId;
    fn expected_length(&self) -> usize {
        10
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        ProposalShortId::new_unchecked(inner.into())
    }
}
impl ProposalShortIdBuilder {
    pub const NAME: &'static str = "ProposalShortIdBuilder";
    pub fn set(mut self, v: [u8; 10]) -> Self {
        self.0 = v;
        self
    }
    pub fn nth0(mut self, v: u8) -> Self {
        self.0[0] = v;
        self
    }
    pub fn nth1(mut self, v: u8) -> Self {
        self.0[1] = v;
        self
    }
    pub fn nth2(mut self, v: u8) -> Self {
        self.0[2] = v;
        self
    }
    pub fn nth3(mut self, v: u8) -> Self {
        self.0[3] = v;
        self
    }
    pub fn nth4(mut self, v: u8) -> Self {
        self.0[4] = v;
        self
    }
    pub fn nth5(mut self, v: u8) -> Self {
        self.0[5] = v;
        self
    }
    pub fn nth6(mut self, v: u8) -> Self {
        self.0[6] = v;
        self
    }
    pub fn nth7(mut self, v: u8) -> Self {
        self.0[7] = v;
        self
    }
    pub fn nth8(mut self, v: u8) -> Self {
        self.0[8] = v;
        self
    }
    pub fn nth9(mut self, v: u8) -> Self {
        self.0[9] = v;
        self
    }
}
#[derive(Clone)]
pub struct ScriptHashType(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct ScriptHashTypeReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for ScriptHashType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for ScriptHashTypeReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for ScriptHashType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Display for ScriptHashTypeReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(&self.raw_data()).unwrap()
        )
    }
}
pub struct ScriptHashTypeBuilder(pub(crate) [u8; 1]);
impl ::std::fmt::Debug for ScriptHashTypeBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}({:?})", Self::NAME, &self.0[..])
    }
}
impl ::std::default::Default for ScriptHashTypeBuilder {
    fn default() -> Self {
        ScriptHashTypeBuilder([0; 1])
    }
}
impl molecule::prelude::Entity for ScriptHashType {
    type Builder = ScriptHashTypeBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ScriptHashType(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptHashTypeReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set([self.nth0()])
    }
}
impl ::std::default::Default for ScriptHashType {
    fn default() -> Self {
        let v: Vec<u8> = vec![0];
        ScriptHashType::new_unchecked(v.into())
    }
}
impl ScriptHashType {
    pub const NAME: &'static str = "ScriptHashType";
    pub fn as_reader(&self) -> ScriptHashTypeReader<'_> {
        ScriptHashTypeReader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 1;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 1;
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.as_bytes()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
}
impl<'r> molecule::prelude::Reader<'r> for ScriptHashTypeReader<'r> {
    type Entity = ScriptHashType;
    fn to_entity(&self) -> Self::Entity {
        ScriptHashType::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ScriptHashTypeReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 1 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 1, slice.len());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> ScriptHashTypeReader<'r> {
    pub const NAME: &'r str = "ScriptHashTypeReader";
    pub const TOTAL_SIZE: usize = 1;
    pub const ITEM_SIZE: usize = 1;
    pub const ITEM_COUNT: usize = 1;
    pub fn raw_data(&self) -> &[u8] {
        self.as_slice()
    }
    pub fn nth0(&self) -> u8 {
        self.0[0]
    }
}
impl molecule::prelude::Builder for ScriptHashTypeBuilder {
    type Entity = ScriptHashType;
    fn expected_length(&self) -> usize {
        1
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        ScriptHashType::new_unchecked(inner.into())
    }
}
impl ScriptHashTypeBuilder {
    pub const NAME: &'static str = "ScriptHashTypeBuilder";
    pub fn set(mut self, v: [u8; 1]) -> Self {
        self.0 = v;
        self
    }
    pub fn nth0(mut self, v: u8) -> Self {
        self.0[0] = v;
        self
    }
}
#[derive(Clone)]
pub struct HeaderVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct HeaderVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for HeaderVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for HeaderVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for HeaderVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for HeaderVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct HeaderVecBuilder(pub(crate) Vec<Header>);
impl molecule::prelude::Entity for HeaderVec {
    type Builder = HeaderVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        HeaderVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        HeaderVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for HeaderVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        HeaderVec::new_unchecked(v.into())
    }
}
impl HeaderVec {
    pub const NAME: &'static str = "HeaderVec";
    pub fn as_reader(&self) -> HeaderVecReader<'_> {
        HeaderVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Header> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Header {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            Header::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            Header::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for HeaderVecReader<'r> {
    type Entity = HeaderVec;
    fn to_entity(&self) -> Self::Entity {
        HeaderVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        HeaderVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            HeaderReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> HeaderVecReader<'r> {
    pub const NAME: &'r str = "HeaderVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<HeaderReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> HeaderReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            HeaderReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            HeaderReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for HeaderVecBuilder {
    type Entity = HeaderVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        HeaderVec::new_unchecked(inner.into())
    }
}
impl HeaderVecBuilder {
    pub const NAME: &'static str = "HeaderVecBuilder";
    pub fn set(mut self, v: Vec<Header>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Header) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Header>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct HeaderVecIterator(HeaderVec, usize, usize);
impl ::std::iter::Iterator for HeaderVecIterator {
    type Item = Header;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for HeaderVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for HeaderVec {
    type Item = Header;
    type IntoIter = HeaderVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        HeaderVecIterator(self, 0, len)
    }
}
impl<'r> HeaderVecReader<'r> {
    pub fn iter(&self) -> HeaderVecReaderIterator<'_, 'r> {
        HeaderVecReaderIterator(&self, 0, self.len())
    }
}
pub struct HeaderVecReaderIterator<'t, 'r>(&'t HeaderVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for HeaderVecReaderIterator<'t, 'r> {
    type Item = HeaderReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for HeaderVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct UncleBlockVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct UncleBlockVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for UncleBlockVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for UncleBlockVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for UncleBlockVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for UncleBlockVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct UncleBlockVecBuilder(pub(crate) Vec<UncleBlock>);
impl molecule::prelude::Entity for UncleBlockVec {
    type Builder = UncleBlockVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        UncleBlockVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        UncleBlockVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for UncleBlockVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        UncleBlockVec::new_unchecked(v.into())
    }
}
impl UncleBlockVec {
    pub const NAME: &'static str = "UncleBlockVec";
    pub fn as_reader(&self) -> UncleBlockVecReader<'_> {
        UncleBlockVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<UncleBlock> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> UncleBlock {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            UncleBlock::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            UncleBlock::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for UncleBlockVecReader<'r> {
    type Entity = UncleBlockVec;
    fn to_entity(&self) -> Self::Entity {
        UncleBlockVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        UncleBlockVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            UncleBlockReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> UncleBlockVecReader<'r> {
    pub const NAME: &'r str = "UncleBlockVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<UncleBlockReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> UncleBlockReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            UncleBlockReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            UncleBlockReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for UncleBlockVecBuilder {
    type Entity = UncleBlockVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        UncleBlockVec::new_unchecked(inner.into())
    }
}
impl UncleBlockVecBuilder {
    pub const NAME: &'static str = "UncleBlockVecBuilder";
    pub fn set(mut self, v: Vec<UncleBlock>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: UncleBlock) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = UncleBlock>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct UncleBlockVecIterator(UncleBlockVec, usize, usize);
impl ::std::iter::Iterator for UncleBlockVecIterator {
    type Item = UncleBlock;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for UncleBlockVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for UncleBlockVec {
    type Item = UncleBlock;
    type IntoIter = UncleBlockVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        UncleBlockVecIterator(self, 0, len)
    }
}
impl<'r> UncleBlockVecReader<'r> {
    pub fn iter(&self) -> UncleBlockVecReaderIterator<'_, 'r> {
        UncleBlockVecReaderIterator(&self, 0, self.len())
    }
}
pub struct UncleBlockVecReaderIterator<'t, 'r>(&'t UncleBlockVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for UncleBlockVecReaderIterator<'t, 'r> {
    type Item = UncleBlockReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for UncleBlockVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct TransactionVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct TransactionVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for TransactionVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for TransactionVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for TransactionVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for TransactionVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct TransactionVecBuilder(pub(crate) Vec<Transaction>);
impl molecule::prelude::Entity for TransactionVec {
    type Builder = TransactionVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        TransactionVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        TransactionVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for TransactionVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        TransactionVec::new_unchecked(v.into())
    }
}
impl TransactionVec {
    pub const NAME: &'static str = "TransactionVec";
    pub fn as_reader(&self) -> TransactionVecReader<'_> {
        TransactionVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Transaction> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Transaction {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            Transaction::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            Transaction::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for TransactionVecReader<'r> {
    type Entity = TransactionVec;
    fn to_entity(&self) -> Self::Entity {
        TransactionVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        TransactionVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            TransactionReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> TransactionVecReader<'r> {
    pub const NAME: &'r str = "TransactionVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<TransactionReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> TransactionReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            TransactionReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            TransactionReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for TransactionVecBuilder {
    type Entity = TransactionVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        TransactionVec::new_unchecked(inner.into())
    }
}
impl TransactionVecBuilder {
    pub const NAME: &'static str = "TransactionVecBuilder";
    pub fn set(mut self, v: Vec<Transaction>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Transaction) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Transaction>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct TransactionVecIterator(TransactionVec, usize, usize);
impl ::std::iter::Iterator for TransactionVecIterator {
    type Item = Transaction;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for TransactionVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for TransactionVec {
    type Item = Transaction;
    type IntoIter = TransactionVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        TransactionVecIterator(self, 0, len)
    }
}
impl<'r> TransactionVecReader<'r> {
    pub fn iter(&self) -> TransactionVecReaderIterator<'_, 'r> {
        TransactionVecReaderIterator(&self, 0, self.len())
    }
}
pub struct TransactionVecReaderIterator<'t, 'r>(&'t TransactionVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for TransactionVecReaderIterator<'t, 'r> {
    type Item = TransactionReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for TransactionVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct ProposalShortIdVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct ProposalShortIdVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for ProposalShortIdVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for ProposalShortIdVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for ProposalShortIdVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for ProposalShortIdVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct ProposalShortIdVecBuilder(pub(crate) Vec<ProposalShortId>);
impl molecule::prelude::Entity for ProposalShortIdVec {
    type Builder = ProposalShortIdVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ProposalShortIdVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ProposalShortIdVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for ProposalShortIdVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        ProposalShortIdVec::new_unchecked(v.into())
    }
}
impl ProposalShortIdVec {
    pub const NAME: &'static str = "ProposalShortIdVec";
    pub fn as_reader(&self) -> ProposalShortIdVecReader<'_> {
        ProposalShortIdVecReader::new_unchecked(self.as_slice())
    }
    pub const ITEM_SIZE: usize = 10;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<ProposalShortId> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> ProposalShortId {
        let start = 4 + idx * 10;
        let end = start + 10;
        ProposalShortId::new_unchecked(self.0.slice(start, end))
    }
}
impl<'r> molecule::prelude::Reader<'r> for ProposalShortIdVecReader<'r> {
    type Entity = ProposalShortIdVec;
    fn to_entity(&self) -> Self::Entity {
        ProposalShortIdVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ProposalShortIdVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let item_count = u32::from_le(ptr[0]) as usize;
        let expected = 4 + 10 * item_count;
        if len != expected {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), expected, len);
            Err(err)?;
        }
        for i in 0..item_count {
            let start = 10 * i;
            let end = start + 10;
            ProposalShortIdReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> ProposalShortIdVecReader<'r> {
    pub const NAME: &'r str = "ProposalShortIdVecReader";
    pub const ITEM_SIZE: usize = 10;
    pub fn len(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<ProposalShortIdReader<'_>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> ProposalShortIdReader<'_> {
        let start = 4 + idx * 10;
        let end = start + 10;
        ProposalShortIdReader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl molecule::prelude::Builder for ProposalShortIdVecBuilder {
    type Entity = ProposalShortIdVec;
    fn expected_length(&self) -> usize {
        4 + 10 * self.0.len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.0.len() as u32).to_le_bytes();
        writer.write_all(&len)?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        ProposalShortIdVec::new_unchecked(inner.into())
    }
}
impl ProposalShortIdVecBuilder {
    pub const NAME: &'static str = "ProposalShortIdVecBuilder";
    pub fn set(mut self, v: Vec<ProposalShortId>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: ProposalShortId) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = ProposalShortId>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct ProposalShortIdVecIterator(ProposalShortIdVec, usize, usize);
impl ::std::iter::Iterator for ProposalShortIdVecIterator {
    type Item = ProposalShortId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for ProposalShortIdVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for ProposalShortIdVec {
    type Item = ProposalShortId;
    type IntoIter = ProposalShortIdVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        ProposalShortIdVecIterator(self, 0, len)
    }
}
impl<'r> ProposalShortIdVecReader<'r> {
    pub fn iter(&self) -> ProposalShortIdVecReaderIterator<'_, 'r> {
        ProposalShortIdVecReaderIterator(&self, 0, self.len())
    }
}
pub struct ProposalShortIdVecReaderIterator<'t, 'r>(&'t ProposalShortIdVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for ProposalShortIdVecReaderIterator<'t, 'r> {
    type Item = ProposalShortIdReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for ProposalShortIdVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct OutPointVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct OutPointVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for OutPointVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for OutPointVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for OutPointVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for OutPointVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct OutPointVecBuilder(pub(crate) Vec<OutPoint>);
impl molecule::prelude::Entity for OutPointVec {
    type Builder = OutPointVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        OutPointVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        OutPointVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for OutPointVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        OutPointVec::new_unchecked(v.into())
    }
}
impl OutPointVec {
    pub const NAME: &'static str = "OutPointVec";
    pub fn as_reader(&self) -> OutPointVecReader<'_> {
        OutPointVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<OutPoint> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> OutPoint {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            OutPoint::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            OutPoint::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for OutPointVecReader<'r> {
    type Entity = OutPointVec;
    fn to_entity(&self) -> Self::Entity {
        OutPointVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        OutPointVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            OutPointReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> OutPointVecReader<'r> {
    pub const NAME: &'r str = "OutPointVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<OutPointReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> OutPointReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            OutPointReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            OutPointReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for OutPointVecBuilder {
    type Entity = OutPointVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        OutPointVec::new_unchecked(inner.into())
    }
}
impl OutPointVecBuilder {
    pub const NAME: &'static str = "OutPointVecBuilder";
    pub fn set(mut self, v: Vec<OutPoint>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: OutPoint) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = OutPoint>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct OutPointVecIterator(OutPointVec, usize, usize);
impl ::std::iter::Iterator for OutPointVecIterator {
    type Item = OutPoint;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for OutPointVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for OutPointVec {
    type Item = OutPoint;
    type IntoIter = OutPointVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        OutPointVecIterator(self, 0, len)
    }
}
impl<'r> OutPointVecReader<'r> {
    pub fn iter(&self) -> OutPointVecReaderIterator<'_, 'r> {
        OutPointVecReaderIterator(&self, 0, self.len())
    }
}
pub struct OutPointVecReaderIterator<'t, 'r>(&'t OutPointVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for OutPointVecReaderIterator<'t, 'r> {
    type Item = OutPointReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for OutPointVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct CellInputVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellInputVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellInputVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellInputVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellInputVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for CellInputVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct CellInputVecBuilder(pub(crate) Vec<CellInput>);
impl molecule::prelude::Entity for CellInputVec {
    type Builder = CellInputVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellInputVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellInputVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for CellInputVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        CellInputVec::new_unchecked(v.into())
    }
}
impl CellInputVec {
    pub const NAME: &'static str = "CellInputVec";
    pub fn as_reader(&self) -> CellInputVecReader<'_> {
        CellInputVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<CellInput> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> CellInput {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            CellInput::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            CellInput::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellInputVecReader<'r> {
    type Entity = CellInputVec;
    fn to_entity(&self) -> Self::Entity {
        CellInputVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellInputVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            CellInputReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> CellInputVecReader<'r> {
    pub const NAME: &'r str = "CellInputVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<CellInputReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> CellInputReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            CellInputReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            CellInputReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for CellInputVecBuilder {
    type Entity = CellInputVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellInputVec::new_unchecked(inner.into())
    }
}
impl CellInputVecBuilder {
    pub const NAME: &'static str = "CellInputVecBuilder";
    pub fn set(mut self, v: Vec<CellInput>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: CellInput) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = CellInput>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct CellInputVecIterator(CellInputVec, usize, usize);
impl ::std::iter::Iterator for CellInputVecIterator {
    type Item = CellInput;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for CellInputVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for CellInputVec {
    type Item = CellInput;
    type IntoIter = CellInputVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        CellInputVecIterator(self, 0, len)
    }
}
impl<'r> CellInputVecReader<'r> {
    pub fn iter(&self) -> CellInputVecReaderIterator<'_, 'r> {
        CellInputVecReaderIterator(&self, 0, self.len())
    }
}
pub struct CellInputVecReaderIterator<'t, 'r>(&'t CellInputVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for CellInputVecReaderIterator<'t, 'r> {
    type Item = CellInputReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for CellInputVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct CellOutputVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellOutputVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellOutputVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellOutputVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellOutputVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for CellOutputVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct CellOutputVecBuilder(pub(crate) Vec<CellOutput>);
impl molecule::prelude::Entity for CellOutputVec {
    type Builder = CellOutputVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellOutputVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellOutputVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for CellOutputVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        CellOutputVec::new_unchecked(v.into())
    }
}
impl CellOutputVec {
    pub const NAME: &'static str = "CellOutputVec";
    pub fn as_reader(&self) -> CellOutputVecReader<'_> {
        CellOutputVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<CellOutput> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> CellOutput {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            CellOutput::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            CellOutput::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellOutputVecReader<'r> {
    type Entity = CellOutputVec;
    fn to_entity(&self) -> Self::Entity {
        CellOutputVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellOutputVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            CellOutputReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> CellOutputVecReader<'r> {
    pub const NAME: &'r str = "CellOutputVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<CellOutputReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> CellOutputReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            CellOutputReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            CellOutputReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for CellOutputVecBuilder {
    type Entity = CellOutputVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellOutputVec::new_unchecked(inner.into())
    }
}
impl CellOutputVecBuilder {
    pub const NAME: &'static str = "CellOutputVecBuilder";
    pub fn set(mut self, v: Vec<CellOutput>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: CellOutput) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = CellOutput>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct CellOutputVecIterator(CellOutputVec, usize, usize);
impl ::std::iter::Iterator for CellOutputVecIterator {
    type Item = CellOutput;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for CellOutputVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for CellOutputVec {
    type Item = CellOutput;
    type IntoIter = CellOutputVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        CellOutputVecIterator(self, 0, len)
    }
}
impl<'r> CellOutputVecReader<'r> {
    pub fn iter(&self) -> CellOutputVecReaderIterator<'_, 'r> {
        CellOutputVecReaderIterator(&self, 0, self.len())
    }
}
pub struct CellOutputVecReaderIterator<'t, 'r>(&'t CellOutputVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for CellOutputVecReaderIterator<'t, 'r> {
    type Item = CellOutputReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for CellOutputVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct WitnessVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct WitnessVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for WitnessVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for WitnessVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for WitnessVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for WitnessVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct WitnessVecBuilder(pub(crate) Vec<Witness>);
impl molecule::prelude::Entity for WitnessVec {
    type Builder = WitnessVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        WitnessVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        WitnessVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for WitnessVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        WitnessVec::new_unchecked(v.into())
    }
}
impl WitnessVec {
    pub const NAME: &'static str = "WitnessVec";
    pub fn as_reader(&self) -> WitnessVecReader<'_> {
        WitnessVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Witness> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Witness {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            Witness::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            Witness::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for WitnessVecReader<'r> {
    type Entity = WitnessVec;
    fn to_entity(&self) -> Self::Entity {
        WitnessVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        WitnessVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            WitnessReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> WitnessVecReader<'r> {
    pub const NAME: &'r str = "WitnessVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<WitnessReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> WitnessReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            WitnessReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            WitnessReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for WitnessVecBuilder {
    type Entity = WitnessVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        WitnessVec::new_unchecked(inner.into())
    }
}
impl WitnessVecBuilder {
    pub const NAME: &'static str = "WitnessVecBuilder";
    pub fn set(mut self, v: Vec<Witness>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Witness) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Witness>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct WitnessVecIterator(WitnessVec, usize, usize);
impl ::std::iter::Iterator for WitnessVecIterator {
    type Item = Witness;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for WitnessVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for WitnessVec {
    type Item = Witness;
    type IntoIter = WitnessVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        WitnessVecIterator(self, 0, len)
    }
}
impl<'r> WitnessVecReader<'r> {
    pub fn iter(&self) -> WitnessVecReaderIterator<'_, 'r> {
        WitnessVecReaderIterator(&self, 0, self.len())
    }
}
pub struct WitnessVecReaderIterator<'t, 'r>(&'t WitnessVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for WitnessVecReaderIterator<'t, 'r> {
    type Item = WitnessReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for WitnessVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct Script(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct ScriptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Script {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for ScriptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Script {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "code_hash", self.code_hash())?;
        write!(f, ", {}: {}", "args", self.args())?;
        write!(f, ", {}: {}", "hash_type", self.hash_type())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for ScriptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "code_hash", self.code_hash())?;
        write!(f, ", {}: {}", "args", self.args())?;
        write!(f, ", {}: {}", "hash_type", self.hash_type())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct ScriptBuilder {
    pub(crate) code_hash: Byte32,
    pub(crate) args: BytesVec,
    pub(crate) hash_type: ScriptHashType,
}
impl ::std::default::Default for Script {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            53, 0, 0, 0, 16, 0, 0, 0, 48, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
        ];
        Script::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Script {
    type Builder = ScriptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Script(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .code_hash(self.code_hash())
            .args(self.args())
            .hash_type(self.hash_type())
    }
}
impl Script {
    pub const NAME: &'static str = "Script";
    pub fn as_reader(&self) -> ScriptReader<'_> {
        ScriptReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn code_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn args(&self) -> BytesVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        BytesVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn hash_type(&self) -> ScriptHashType {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            ScriptHashType::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            ScriptHashType::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ScriptReader<'r> {
    type Entity = Script;
    fn to_entity(&self) -> Self::Entity {
        Script::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ScriptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        BytesVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        ScriptHashTypeReader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> ScriptReader<'r> {
    pub const NAME: &'r str = "ScriptReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn code_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn args(&self) -> BytesVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        BytesVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn hash_type(&self) -> ScriptHashTypeReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            ScriptHashTypeReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            ScriptHashTypeReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for ScriptBuilder {
    type Entity = Script;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header
            + self.code_hash.as_slice().len()
            + self.args.as_slice().len()
            + self.hash_type.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.code_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.args.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.hash_type.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.code_hash.as_slice())?;
        writer.write_all(self.args.as_slice())?;
        writer.write_all(self.hash_type.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Script::new_unchecked(inner.into())
    }
}
impl ScriptBuilder {
    pub const NAME: &'static str = "ScriptBuilder";
    pub fn code_hash(mut self, v: Byte32) -> Self {
        self.code_hash = v;
        self
    }
    pub fn args(mut self, v: BytesVec) -> Self {
        self.args = v;
        self
    }
    pub fn hash_type(mut self, v: ScriptHashType) -> Self {
        self.hash_type = v;
        self
    }
}
#[derive(Clone)]
pub struct CellOutPoint(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellOutPointReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellOutPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellOutPointReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellOutPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "tx_hash", self.tx_hash())?;
        write!(f, ", {}: {}", "index", self.index())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for CellOutPointReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "tx_hash", self.tx_hash())?;
        write!(f, ", {}: {}", "index", self.index())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct CellOutPointBuilder {
    pub(crate) tx_hash: Byte32,
    pub(crate) index: Uint32,
}
impl ::std::default::Default for CellOutPoint {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            48, 0, 0, 0, 12, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        CellOutPoint::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for CellOutPoint {
    type Builder = CellOutPointBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellOutPoint(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellOutPointReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .tx_hash(self.tx_hash())
            .index(self.index())
    }
}
impl CellOutPoint {
    pub const NAME: &'static str = "CellOutPoint";
    pub fn as_reader(&self) -> CellOutPointReader<'_> {
        CellOutPointReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn tx_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn index(&self) -> Uint32 {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Uint32::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Uint32::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellOutPointReader<'r> {
    type Entity = CellOutPoint;
    fn to_entity(&self) -> Self::Entity {
        CellOutPoint::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellOutPointReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Uint32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> CellOutPointReader<'r> {
    pub const NAME: &'r str = "CellOutPointReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn tx_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn index(&self) -> Uint32Reader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Uint32Reader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Uint32Reader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for CellOutPointBuilder {
    type Entity = CellOutPoint;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.tx_hash.as_slice().len() + self.index.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.tx_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.index.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.tx_hash.as_slice())?;
        writer.write_all(self.index.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellOutPoint::new_unchecked(inner.into())
    }
}
impl CellOutPointBuilder {
    pub const NAME: &'static str = "CellOutPointBuilder";
    pub fn tx_hash(mut self, v: Byte32) -> Self {
        self.tx_hash = v;
        self
    }
    pub fn index(mut self, v: Uint32) -> Self {
        self.index = v;
        self
    }
}
#[derive(Clone)]
pub struct OutPoint(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct OutPointReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for OutPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for OutPointReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for OutPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "cell", self.cell())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for OutPointReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "cell", self.cell())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct OutPointBuilder {
    pub(crate) cell: CellOutPointOpt,
    pub(crate) block_hash: Byte32Opt,
}
impl ::std::default::Default for OutPoint {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 12, 0, 0, 0, 12, 0, 0, 0];
        OutPoint::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for OutPoint {
    type Builder = OutPointBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        OutPoint(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        OutPointReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .cell(self.cell())
            .block_hash(self.block_hash())
    }
}
impl OutPoint {
    pub const NAME: &'static str = "OutPoint";
    pub fn as_reader(&self) -> OutPointReader<'_> {
        OutPointReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn cell(&self) -> CellOutPointOpt {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        CellOutPointOpt::new_unchecked(self.0.slice(start, end))
    }
    pub fn block_hash(&self) -> Byte32Opt {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32Opt::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32Opt::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for OutPointReader<'r> {
    type Entity = OutPoint;
    fn to_entity(&self) -> Self::Entity {
        OutPoint::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        OutPointReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        CellOutPointOptReader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32OptReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> OutPointReader<'r> {
    pub const NAME: &'r str = "OutPointReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn cell(&self) -> CellOutPointOptReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        CellOutPointOptReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn block_hash(&self) -> Byte32OptReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32OptReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32OptReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for OutPointBuilder {
    type Entity = OutPoint;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.cell.as_slice().len() + self.block_hash.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.cell.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_hash.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.cell.as_slice())?;
        writer.write_all(self.block_hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        OutPoint::new_unchecked(inner.into())
    }
}
impl OutPointBuilder {
    pub const NAME: &'static str = "OutPointBuilder";
    pub fn cell(mut self, v: CellOutPointOpt) -> Self {
        self.cell = v;
        self
    }
    pub fn block_hash(mut self, v: Byte32Opt) -> Self {
        self.block_hash = v;
        self
    }
}
#[derive(Clone)]
pub struct CellInput(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellInputReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellInputReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "previous_output", self.previous_output())?;
        write!(f, ", {}: {}", "since", self.since())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for CellInputReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "previous_output", self.previous_output())?;
        write!(f, ", {}: {}", "since", self.since())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct CellInputBuilder {
    pub(crate) previous_output: OutPoint,
    pub(crate) since: Uint64,
}
impl ::std::default::Default for CellInput {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            32, 0, 0, 0, 12, 0, 0, 0, 24, 0, 0, 0, 12, 0, 0, 0, 12, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        CellInput::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for CellInput {
    type Builder = CellInputBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellInput(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellInputReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .previous_output(self.previous_output())
            .since(self.since())
    }
}
impl CellInput {
    pub const NAME: &'static str = "CellInput";
    pub fn as_reader(&self) -> CellInputReader<'_> {
        CellInputReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn previous_output(&self) -> OutPoint {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        OutPoint::new_unchecked(self.0.slice(start, end))
    }
    pub fn since(&self) -> Uint64 {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Uint64::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Uint64::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellInputReader<'r> {
    type Entity = CellInput;
    fn to_entity(&self) -> Self::Entity {
        CellInput::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellInputReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        OutPointReader::verify(&slice[offsets[0]..offsets[1]])?;
        Uint64Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> CellInputReader<'r> {
    pub const NAME: &'r str = "CellInputReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn previous_output(&self) -> OutPointReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        OutPointReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn since(&self) -> Uint64Reader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Uint64Reader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Uint64Reader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for CellInputBuilder {
    type Entity = CellInput;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.previous_output.as_slice().len() + self.since.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.previous_output.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.since.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.previous_output.as_slice())?;
        writer.write_all(self.since.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellInput::new_unchecked(inner.into())
    }
}
impl CellInputBuilder {
    pub const NAME: &'static str = "CellInputBuilder";
    pub fn previous_output(mut self, v: OutPoint) -> Self {
        self.previous_output = v;
        self
    }
    pub fn since(mut self, v: Uint64) -> Self {
        self.since = v;
        self
    }
}
#[derive(Clone)]
pub struct CellOutput(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellOutputReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellOutputReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "capacity", self.capacity())?;
        write!(f, ", {}: {}", "data_hash", self.data_hash())?;
        write!(f, ", {}: {}", "lock", self.lock())?;
        write!(f, ", {}: {}", "type_", self.type_())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 4 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for CellOutputReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "capacity", self.capacity())?;
        write!(f, ", {}: {}", "data_hash", self.data_hash())?;
        write!(f, ", {}: {}", "lock", self.lock())?;
        write!(f, ", {}: {}", "type_", self.type_())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 4 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct CellOutputBuilder {
    pub(crate) capacity: Uint64,
    pub(crate) data_hash: Byte32,
    pub(crate) lock: Script,
    pub(crate) type_: ScriptOpt,
}
impl ::std::default::Default for CellOutput {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            113, 0, 0, 0, 20, 0, 0, 0, 28, 0, 0, 0, 60, 0, 0, 0, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 53, 0, 0, 0, 16, 0, 0, 0, 48, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
        ];
        CellOutput::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for CellOutput {
    type Builder = CellOutputBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellOutput(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellOutputReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .capacity(self.capacity())
            .data_hash(self.data_hash())
            .lock(self.lock())
            .type_(self.type_())
    }
}
impl CellOutput {
    pub const NAME: &'static str = "CellOutput";
    pub fn as_reader(&self) -> CellOutputReader<'_> {
        CellOutputReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 4;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn capacity(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn data_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn lock(&self) -> Script {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        Script::new_unchecked(self.0.slice(start, end))
    }
    pub fn type_(&self) -> ScriptOpt {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        if count == 4 {
            ScriptOpt::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[3 + 1]) as usize;
            ScriptOpt::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellOutputReader<'r> {
    type Entity = CellOutput;
    fn to_entity(&self) -> Self::Entity {
        CellOutput::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellOutputReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 4;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=4]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        ScriptReader::verify(&slice[offsets[2]..offsets[3]])?;
        ScriptOptReader::verify(&slice[offsets[3]..offsets[4]])?;
        Ok(())
    }
}
impl<'r> CellOutputReader<'r> {
    pub const NAME: &'r str = "CellOutputReader";
    pub const FIELD_COUNT: usize = 4;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn capacity(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn data_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn lock(&self) -> ScriptReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        ScriptReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn type_(&self) -> ScriptOptReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        if count == 4 {
            ScriptOptReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[3 + 1]) as usize;
            ScriptOptReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for CellOutputBuilder {
    type Entity = CellOutput;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * 4;
        len_header
            + self.capacity.as_slice().len()
            + self.data_hash.as_slice().len()
            + self.lock.as_slice().len()
            + self.type_.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.capacity.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.data_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.lock.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.type_.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.capacity.as_slice())?;
        writer.write_all(self.data_hash.as_slice())?;
        writer.write_all(self.lock.as_slice())?;
        writer.write_all(self.type_.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellOutput::new_unchecked(inner.into())
    }
}
impl CellOutputBuilder {
    pub const NAME: &'static str = "CellOutputBuilder";
    pub fn capacity(mut self, v: Uint64) -> Self {
        self.capacity = v;
        self
    }
    pub fn data_hash(mut self, v: Byte32) -> Self {
        self.data_hash = v;
        self
    }
    pub fn lock(mut self, v: Script) -> Self {
        self.lock = v;
        self
    }
    pub fn type_(mut self, v: ScriptOpt) -> Self {
        self.type_ = v;
        self
    }
}
#[derive(Clone)]
pub struct Witness(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct WitnessReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Witness {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for WitnessReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Witness {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for WitnessReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct WitnessBuilder(pub(crate) Vec<Bytes>);
impl molecule::prelude::Entity for Witness {
    type Builder = WitnessBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Witness(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        WitnessReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for Witness {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        Witness::new_unchecked(v.into())
    }
}
impl Witness {
    pub const NAME: &'static str = "Witness";
    pub fn as_reader(&self) -> WitnessReader<'_> {
        WitnessReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Bytes> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Bytes {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for WitnessReader<'r> {
    type Entity = Witness;
    fn to_entity(&self) -> Self::Entity {
        Witness::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        WitnessReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            BytesReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> WitnessReader<'r> {
    pub const NAME: &'r str = "WitnessReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<BytesReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> BytesReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for WitnessBuilder {
    type Entity = Witness;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Witness::new_unchecked(inner.into())
    }
}
impl WitnessBuilder {
    pub const NAME: &'static str = "WitnessBuilder";
    pub fn set(mut self, v: Vec<Bytes>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Bytes) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = Bytes>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct WitnessIterator(Witness, usize, usize);
impl ::std::iter::Iterator for WitnessIterator {
    type Item = Bytes;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for WitnessIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for Witness {
    type Item = Bytes;
    type IntoIter = WitnessIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        WitnessIterator(self, 0, len)
    }
}
impl<'r> WitnessReader<'r> {
    pub fn iter(&self) -> WitnessReaderIterator<'_, 'r> {
        WitnessReaderIterator(&self, 0, self.len())
    }
}
pub struct WitnessReaderIterator<'t, 'r>(&'t WitnessReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::std::iter::Iterator for WitnessReaderIterator<'t, 'r> {
    type Item = BytesReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for WitnessReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct RawTransaction(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RawTransactionReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RawTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RawTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RawTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "deps", self.deps())?;
        write!(f, ", {}: {}", "inputs", self.inputs())?;
        write!(f, ", {}: {}", "outputs", self.outputs())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 4 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for RawTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "deps", self.deps())?;
        write!(f, ", {}: {}", "inputs", self.inputs())?;
        write!(f, ", {}: {}", "outputs", self.outputs())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 4 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct RawTransactionBuilder {
    pub(crate) version: Uint32,
    pub(crate) deps: OutPointVec,
    pub(crate) inputs: CellInputVec,
    pub(crate) outputs: CellOutputVec,
}
impl ::std::default::Default for RawTransaction {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            36, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 28, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
            0, 4, 0, 0, 0, 4, 0, 0, 0,
        ];
        RawTransaction::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for RawTransaction {
    type Builder = RawTransactionBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RawTransaction(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RawTransactionReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .version(self.version())
            .deps(self.deps())
            .inputs(self.inputs())
            .outputs(self.outputs())
    }
}
impl RawTransaction {
    pub const NAME: &'static str = "RawTransaction";
    pub fn as_reader(&self) -> RawTransactionReader<'_> {
        RawTransactionReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 4;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn version(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn deps(&self) -> OutPointVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        OutPointVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn inputs(&self) -> CellInputVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        CellInputVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn outputs(&self) -> CellOutputVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        if count == 4 {
            CellOutputVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[3 + 1]) as usize;
            CellOutputVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RawTransactionReader<'r> {
    type Entity = RawTransaction;
    fn to_entity(&self) -> Self::Entity {
        RawTransaction::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RawTransactionReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 4;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=4]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        OutPointVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        CellInputVecReader::verify(&slice[offsets[2]..offsets[3]])?;
        CellOutputVecReader::verify(&slice[offsets[3]..offsets[4]])?;
        Ok(())
    }
}
impl<'r> RawTransactionReader<'r> {
    pub const NAME: &'r str = "RawTransactionReader";
    pub const FIELD_COUNT: usize = 4;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn version(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn deps(&self) -> OutPointVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        OutPointVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn inputs(&self) -> CellInputVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        CellInputVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn outputs(&self) -> CellOutputVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        if count == 4 {
            CellOutputVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[3 + 1]) as usize;
            CellOutputVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for RawTransactionBuilder {
    type Entity = RawTransaction;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * 4;
        len_header
            + self.version.as_slice().len()
            + self.deps.as_slice().len()
            + self.inputs.as_slice().len()
            + self.outputs.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.version.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.deps.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.inputs.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.outputs.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.version.as_slice())?;
        writer.write_all(self.deps.as_slice())?;
        writer.write_all(self.inputs.as_slice())?;
        writer.write_all(self.outputs.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RawTransaction::new_unchecked(inner.into())
    }
}
impl RawTransactionBuilder {
    pub const NAME: &'static str = "RawTransactionBuilder";
    pub fn version(mut self, v: Uint32) -> Self {
        self.version = v;
        self
    }
    pub fn deps(mut self, v: OutPointVec) -> Self {
        self.deps = v;
        self
    }
    pub fn inputs(mut self, v: CellInputVec) -> Self {
        self.inputs = v;
        self
    }
    pub fn outputs(mut self, v: CellOutputVec) -> Self {
        self.outputs = v;
        self
    }
}
#[derive(Clone)]
pub struct SlimTransaction(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct SlimTransactionReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for SlimTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for SlimTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for SlimTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "raw", self.raw())?;
        write!(f, ", {}: {}", "witnesses", self.witnesses())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for SlimTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "raw", self.raw())?;
        write!(f, ", {}: {}", "witnesses", self.witnesses())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct SlimTransactionBuilder {
    pub(crate) raw: RawTransaction,
    pub(crate) witnesses: WitnessVec,
}
impl ::std::default::Default for SlimTransaction {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            52, 0, 0, 0, 12, 0, 0, 0, 48, 0, 0, 0, 36, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 28, 0, 0,
            0, 32, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0,
        ];
        SlimTransaction::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for SlimTransaction {
    type Builder = SlimTransactionBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SlimTransaction(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SlimTransactionReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .raw(self.raw())
            .witnesses(self.witnesses())
    }
}
impl SlimTransaction {
    pub const NAME: &'static str = "SlimTransaction";
    pub fn as_reader(&self) -> SlimTransactionReader<'_> {
        SlimTransactionReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn raw(&self) -> RawTransaction {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        RawTransaction::new_unchecked(self.0.slice(start, end))
    }
    pub fn witnesses(&self) -> WitnessVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            WitnessVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            WitnessVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for SlimTransactionReader<'r> {
    type Entity = SlimTransaction;
    fn to_entity(&self) -> Self::Entity {
        SlimTransaction::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        SlimTransactionReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        RawTransactionReader::verify(&slice[offsets[0]..offsets[1]])?;
        WitnessVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> SlimTransactionReader<'r> {
    pub const NAME: &'r str = "SlimTransactionReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn raw(&self) -> RawTransactionReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        RawTransactionReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn witnesses(&self) -> WitnessVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            WitnessVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            WitnessVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for SlimTransactionBuilder {
    type Entity = SlimTransaction;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.raw.as_slice().len() + self.witnesses.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.raw.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.witnesses.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.raw.as_slice())?;
        writer.write_all(self.witnesses.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        SlimTransaction::new_unchecked(inner.into())
    }
}
impl SlimTransactionBuilder {
    pub const NAME: &'static str = "SlimTransactionBuilder";
    pub fn raw(mut self, v: RawTransaction) -> Self {
        self.raw = v;
        self
    }
    pub fn witnesses(mut self, v: WitnessVec) -> Self {
        self.witnesses = v;
        self
    }
}
#[derive(Clone)]
pub struct Transaction(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct TransactionReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for TransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "slim", self.slim())?;
        write!(f, ", {}: {}", "outputs_data", self.outputs_data())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for TransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "slim", self.slim())?;
        write!(f, ", {}: {}", "outputs_data", self.outputs_data())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct TransactionBuilder {
    pub(crate) slim: SlimTransaction,
    pub(crate) outputs_data: BytesVec,
}
impl ::std::default::Default for Transaction {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            68, 0, 0, 0, 12, 0, 0, 0, 64, 0, 0, 0, 52, 0, 0, 0, 12, 0, 0, 0, 48, 0, 0, 0, 36, 0, 0,
            0, 20, 0, 0, 0, 24, 0, 0, 0, 28, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0,
            0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0,
        ];
        Transaction::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Transaction {
    type Builder = TransactionBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Transaction(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        TransactionReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .slim(self.slim())
            .outputs_data(self.outputs_data())
    }
}
impl Transaction {
    pub const NAME: &'static str = "Transaction";
    pub fn as_reader(&self) -> TransactionReader<'_> {
        TransactionReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn slim(&self) -> SlimTransaction {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        SlimTransaction::new_unchecked(self.0.slice(start, end))
    }
    pub fn outputs_data(&self) -> BytesVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            BytesVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            BytesVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for TransactionReader<'r> {
    type Entity = Transaction;
    fn to_entity(&self) -> Self::Entity {
        Transaction::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        TransactionReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        SlimTransactionReader::verify(&slice[offsets[0]..offsets[1]])?;
        BytesVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> TransactionReader<'r> {
    pub const NAME: &'r str = "TransactionReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn slim(&self) -> SlimTransactionReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        SlimTransactionReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn outputs_data(&self) -> BytesVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            BytesVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            BytesVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for TransactionBuilder {
    type Entity = Transaction;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.slim.as_slice().len() + self.outputs_data.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.slim.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.outputs_data.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.slim.as_slice())?;
        writer.write_all(self.outputs_data.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Transaction::new_unchecked(inner.into())
    }
}
impl TransactionBuilder {
    pub const NAME: &'static str = "TransactionBuilder";
    pub fn slim(mut self, v: SlimTransaction) -> Self {
        self.slim = v;
        self
    }
    pub fn outputs_data(mut self, v: BytesVec) -> Self {
        self.outputs_data = v;
        self
    }
}
#[derive(Clone)]
pub struct RawHeader(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RawHeaderReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RawHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RawHeaderReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RawHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "parent_hash", self.parent_hash())?;
        write!(f, ", {}: {}", "timestamp", self.timestamp())?;
        write!(f, ", {}: {}", "number", self.number())?;
        write!(f, ", {}: {}", "transactions_root", self.transactions_root())?;
        write!(f, ", {}: {}", "witnesses_root", self.witnesses_root())?;
        write!(f, ", {}: {}", "proposals_hash", self.proposals_hash())?;
        write!(f, ", {}: {}", "difficulty", self.difficulty())?;
        write!(f, ", {}: {}", "uncles_hash", self.uncles_hash())?;
        write!(f, ", {}: {}", "uncles_count", self.uncles_count())?;
        write!(f, ", {}: {}", "epoch", self.epoch())?;
        write!(f, ", {}: {}", "dao", self.dao())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 12 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for RawHeaderReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "parent_hash", self.parent_hash())?;
        write!(f, ", {}: {}", "timestamp", self.timestamp())?;
        write!(f, ", {}: {}", "number", self.number())?;
        write!(f, ", {}: {}", "transactions_root", self.transactions_root())?;
        write!(f, ", {}: {}", "witnesses_root", self.witnesses_root())?;
        write!(f, ", {}: {}", "proposals_hash", self.proposals_hash())?;
        write!(f, ", {}: {}", "difficulty", self.difficulty())?;
        write!(f, ", {}: {}", "uncles_hash", self.uncles_hash())?;
        write!(f, ", {}: {}", "uncles_count", self.uncles_count())?;
        write!(f, ", {}: {}", "epoch", self.epoch())?;
        write!(f, ", {}: {}", "dao", self.dao())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 12 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct RawHeaderBuilder {
    pub(crate) version: Uint32,
    pub(crate) parent_hash: Byte32,
    pub(crate) timestamp: Uint64,
    pub(crate) number: Uint64,
    pub(crate) transactions_root: Byte32,
    pub(crate) witnesses_root: Byte32,
    pub(crate) proposals_hash: Byte32,
    pub(crate) difficulty: Byte32,
    pub(crate) uncles_hash: Byte32,
    pub(crate) uncles_count: Uint32,
    pub(crate) epoch: Uint64,
    pub(crate) dao: Bytes,
}
impl ::std::default::Default for RawHeader {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            24, 1, 0, 0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0, 0, 0, 96, 0, 0, 0, 104, 0, 0, 0, 136, 0,
            0, 0, 168, 0, 0, 0, 200, 0, 0, 0, 232, 0, 0, 0, 8, 1, 0, 0, 12, 1, 0, 0, 20, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        RawHeader::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for RawHeader {
    type Builder = RawHeaderBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RawHeader(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RawHeaderReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .version(self.version())
            .parent_hash(self.parent_hash())
            .timestamp(self.timestamp())
            .number(self.number())
            .transactions_root(self.transactions_root())
            .witnesses_root(self.witnesses_root())
            .proposals_hash(self.proposals_hash())
            .difficulty(self.difficulty())
            .uncles_hash(self.uncles_hash())
            .uncles_count(self.uncles_count())
            .epoch(self.epoch())
            .dao(self.dao())
    }
}
impl RawHeader {
    pub const NAME: &'static str = "RawHeader";
    pub fn as_reader(&self) -> RawHeaderReader<'_> {
        RawHeaderReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 12;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn version(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn parent_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn timestamp(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn number(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn transactions_root(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        let end = u32::from_le(offsets[4 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn witnesses_root(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[5]) as usize;
        let end = u32::from_le(offsets[5 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn proposals_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[6]) as usize;
        let end = u32::from_le(offsets[6 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn difficulty(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[7]) as usize;
        let end = u32::from_le(offsets[7 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn uncles_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[8]) as usize;
        let end = u32::from_le(offsets[8 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn uncles_count(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[9]) as usize;
        let end = u32::from_le(offsets[9 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn epoch(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[10]) as usize;
        let end = u32::from_le(offsets[10 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn dao(&self) -> Bytes {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[11]) as usize;
        if count == 12 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[11 + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RawHeaderReader<'r> {
    type Entity = RawHeader;
    fn to_entity(&self) -> Self::Entity {
        RawHeader::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RawHeaderReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 12;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=12]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Uint64Reader::verify(&slice[offsets[2]..offsets[3]])?;
        Uint64Reader::verify(&slice[offsets[3]..offsets[4]])?;
        Byte32Reader::verify(&slice[offsets[4]..offsets[5]])?;
        Byte32Reader::verify(&slice[offsets[5]..offsets[6]])?;
        Byte32Reader::verify(&slice[offsets[6]..offsets[7]])?;
        Byte32Reader::verify(&slice[offsets[7]..offsets[8]])?;
        Byte32Reader::verify(&slice[offsets[8]..offsets[9]])?;
        Uint32Reader::verify(&slice[offsets[9]..offsets[10]])?;
        Uint64Reader::verify(&slice[offsets[10]..offsets[11]])?;
        BytesReader::verify(&slice[offsets[11]..offsets[12]])?;
        Ok(())
    }
}
impl<'r> RawHeaderReader<'r> {
    pub const NAME: &'r str = "RawHeaderReader";
    pub const FIELD_COUNT: usize = 12;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn version(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn parent_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn timestamp(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn number(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn transactions_root(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        let end = u32::from_le(offsets[4 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn witnesses_root(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[5]) as usize;
        let end = u32::from_le(offsets[5 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proposals_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[6]) as usize;
        let end = u32::from_le(offsets[6 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn difficulty(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[7]) as usize;
        let end = u32::from_le(offsets[7 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn uncles_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[8]) as usize;
        let end = u32::from_le(offsets[8 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn uncles_count(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[9]) as usize;
        let end = u32::from_le(offsets[9 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn epoch(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[10]) as usize;
        let end = u32::from_le(offsets[10 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn dao(&self) -> BytesReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[11]) as usize;
        if count == 12 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[11 + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for RawHeaderBuilder {
    type Entity = RawHeader;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 12 * 4;
        len_header
            + self.version.as_slice().len()
            + self.parent_hash.as_slice().len()
            + self.timestamp.as_slice().len()
            + self.number.as_slice().len()
            + self.transactions_root.as_slice().len()
            + self.witnesses_root.as_slice().len()
            + self.proposals_hash.as_slice().len()
            + self.difficulty.as_slice().len()
            + self.uncles_hash.as_slice().len()
            + self.uncles_count.as_slice().len()
            + self.epoch.as_slice().len()
            + self.dao.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 12 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.version.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.parent_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.timestamp.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.number.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transactions_root.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.witnesses_root.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.proposals_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.difficulty.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.uncles_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.uncles_count.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.epoch.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.dao.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.version.as_slice())?;
        writer.write_all(self.parent_hash.as_slice())?;
        writer.write_all(self.timestamp.as_slice())?;
        writer.write_all(self.number.as_slice())?;
        writer.write_all(self.transactions_root.as_slice())?;
        writer.write_all(self.witnesses_root.as_slice())?;
        writer.write_all(self.proposals_hash.as_slice())?;
        writer.write_all(self.difficulty.as_slice())?;
        writer.write_all(self.uncles_hash.as_slice())?;
        writer.write_all(self.uncles_count.as_slice())?;
        writer.write_all(self.epoch.as_slice())?;
        writer.write_all(self.dao.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RawHeader::new_unchecked(inner.into())
    }
}
impl RawHeaderBuilder {
    pub const NAME: &'static str = "RawHeaderBuilder";
    pub fn version(mut self, v: Uint32) -> Self {
        self.version = v;
        self
    }
    pub fn parent_hash(mut self, v: Byte32) -> Self {
        self.parent_hash = v;
        self
    }
    pub fn timestamp(mut self, v: Uint64) -> Self {
        self.timestamp = v;
        self
    }
    pub fn number(mut self, v: Uint64) -> Self {
        self.number = v;
        self
    }
    pub fn transactions_root(mut self, v: Byte32) -> Self {
        self.transactions_root = v;
        self
    }
    pub fn witnesses_root(mut self, v: Byte32) -> Self {
        self.witnesses_root = v;
        self
    }
    pub fn proposals_hash(mut self, v: Byte32) -> Self {
        self.proposals_hash = v;
        self
    }
    pub fn difficulty(mut self, v: Byte32) -> Self {
        self.difficulty = v;
        self
    }
    pub fn uncles_hash(mut self, v: Byte32) -> Self {
        self.uncles_hash = v;
        self
    }
    pub fn uncles_count(mut self, v: Uint32) -> Self {
        self.uncles_count = v;
        self
    }
    pub fn epoch(mut self, v: Uint64) -> Self {
        self.epoch = v;
        self
    }
    pub fn dao(mut self, v: Bytes) -> Self {
        self.dao = v;
        self
    }
}
#[derive(Clone)]
pub struct Seal(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct SealReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Seal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for SealReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Seal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "nonce", self.nonce())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for SealReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "nonce", self.nonce())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct SealBuilder {
    pub(crate) nonce: Uint64,
    pub(crate) proof: Bytes,
}
impl ::std::default::Default for Seal {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            24, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        Seal::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Seal {
    type Builder = SealBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Seal(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SealReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().nonce(self.nonce()).proof(self.proof())
    }
}
impl Seal {
    pub const NAME: &'static str = "Seal";
    pub fn as_reader(&self) -> SealReader<'_> {
        SealReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn nonce(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn proof(&self) -> Bytes {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for SealReader<'r> {
    type Entity = Seal;
    fn to_entity(&self) -> Self::Entity {
        Seal::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        SealReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        BytesReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> SealReader<'r> {
    pub const NAME: &'r str = "SealReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn nonce(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proof(&self) -> BytesReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for SealBuilder {
    type Entity = Seal;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.nonce.as_slice().len() + self.proof.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.nonce.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.proof.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.nonce.as_slice())?;
        writer.write_all(self.proof.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Seal::new_unchecked(inner.into())
    }
}
impl SealBuilder {
    pub const NAME: &'static str = "SealBuilder";
    pub fn nonce(mut self, v: Uint64) -> Self {
        self.nonce = v;
        self
    }
    pub fn proof(mut self, v: Bytes) -> Self {
        self.proof = v;
        self
    }
}
#[derive(Clone)]
pub struct Header(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct HeaderReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for HeaderReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "raw", self.raw())?;
        write!(f, ", {}: {}", "seal", self.seal())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for HeaderReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "raw", self.raw())?;
        write!(f, ", {}: {}", "seal", self.seal())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct HeaderBuilder {
    pub(crate) raw: RawHeader,
    pub(crate) seal: Seal,
}
impl ::std::default::Default for Header {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            60, 1, 0, 0, 12, 0, 0, 0, 36, 1, 0, 0, 24, 1, 0, 0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0, 0,
            0, 96, 0, 0, 0, 104, 0, 0, 0, 136, 0, 0, 0, 168, 0, 0, 0, 200, 0, 0, 0, 232, 0, 0, 0,
            8, 1, 0, 0, 12, 1, 0, 0, 20, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        Header::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Header {
    type Builder = HeaderBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Header(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        HeaderReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().raw(self.raw()).seal(self.seal())
    }
}
impl Header {
    pub const NAME: &'static str = "Header";
    pub fn as_reader(&self) -> HeaderReader<'_> {
        HeaderReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn raw(&self) -> RawHeader {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        RawHeader::new_unchecked(self.0.slice(start, end))
    }
    pub fn seal(&self) -> Seal {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Seal::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Seal::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for HeaderReader<'r> {
    type Entity = Header;
    fn to_entity(&self) -> Self::Entity {
        Header::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        HeaderReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        RawHeaderReader::verify(&slice[offsets[0]..offsets[1]])?;
        SealReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> HeaderReader<'r> {
    pub const NAME: &'r str = "HeaderReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn raw(&self) -> RawHeaderReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        RawHeaderReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn seal(&self) -> SealReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            SealReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            SealReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for HeaderBuilder {
    type Entity = Header;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.raw.as_slice().len() + self.seal.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.raw.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.seal.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.raw.as_slice())?;
        writer.write_all(self.seal.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Header::new_unchecked(inner.into())
    }
}
impl HeaderBuilder {
    pub const NAME: &'static str = "HeaderBuilder";
    pub fn raw(mut self, v: RawHeader) -> Self {
        self.raw = v;
        self
    }
    pub fn seal(mut self, v: Seal) -> Self {
        self.seal = v;
        self
    }
}
#[derive(Clone)]
pub struct UncleBlock(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct UncleBlockReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for UncleBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for UncleBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for UncleBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for UncleBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct UncleBlockBuilder {
    pub(crate) header: Header,
    pub(crate) proposals: ProposalShortIdVec,
}
impl ::std::default::Default for UncleBlock {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            76, 1, 0, 0, 12, 0, 0, 0, 72, 1, 0, 0, 60, 1, 0, 0, 12, 0, 0, 0, 36, 1, 0, 0, 24, 1, 0,
            0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0, 0, 0, 96, 0, 0, 0, 104, 0, 0, 0, 136, 0, 0, 0, 168,
            0, 0, 0, 200, 0, 0, 0, 232, 0, 0, 0, 8, 1, 0, 0, 12, 1, 0, 0, 20, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 12, 0, 0,
            0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        UncleBlock::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for UncleBlock {
    type Builder = UncleBlockBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        UncleBlock(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        UncleBlockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .header(self.header())
            .proposals(self.proposals())
    }
}
impl UncleBlock {
    pub const NAME: &'static str = "UncleBlock";
    pub fn as_reader(&self) -> UncleBlockReader<'_> {
        UncleBlockReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> Header {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Header::new_unchecked(self.0.slice(start, end))
    }
    pub fn proposals(&self) -> ProposalShortIdVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            ProposalShortIdVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            ProposalShortIdVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for UncleBlockReader<'r> {
    type Entity = UncleBlock;
    fn to_entity(&self) -> Self::Entity {
        UncleBlock::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        UncleBlockReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        HeaderReader::verify(&slice[offsets[0]..offsets[1]])?;
        ProposalShortIdVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> UncleBlockReader<'r> {
    pub const NAME: &'r str = "UncleBlockReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> HeaderReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        HeaderReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proposals(&self) -> ProposalShortIdVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for UncleBlockBuilder {
    type Entity = UncleBlock;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.header.as_slice().len() + self.proposals.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.header.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.proposals.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.header.as_slice())?;
        writer.write_all(self.proposals.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        UncleBlock::new_unchecked(inner.into())
    }
}
impl UncleBlockBuilder {
    pub const NAME: &'static str = "UncleBlockBuilder";
    pub fn header(mut self, v: Header) -> Self {
        self.header = v;
        self
    }
    pub fn proposals(mut self, v: ProposalShortIdVec) -> Self {
        self.proposals = v;
        self
    }
}
#[derive(Clone)]
pub struct Block(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BlockReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "uncles", self.uncles())?;
        write!(f, ", {}: {}", "transactions", self.transactions())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 4 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for BlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "uncles", self.uncles())?;
        write!(f, ", {}: {}", "transactions", self.transactions())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 4 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct BlockBuilder {
    pub(crate) header: Header,
    pub(crate) uncles: UncleBlockVec,
    pub(crate) transactions: TransactionVec,
    pub(crate) proposals: ProposalShortIdVec,
}
impl ::std::default::Default for Block {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            92, 1, 0, 0, 20, 0, 0, 0, 80, 1, 0, 0, 84, 1, 0, 0, 88, 1, 0, 0, 60, 1, 0, 0, 12, 0, 0,
            0, 36, 1, 0, 0, 24, 1, 0, 0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0, 0, 0, 96, 0, 0, 0, 104,
            0, 0, 0, 136, 0, 0, 0, 168, 0, 0, 0, 200, 0, 0, 0, 232, 0, 0, 0, 8, 1, 0, 0, 12, 1, 0,
            0, 20, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 24, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
            0, 4, 0, 0, 0, 0, 0, 0, 0,
        ];
        Block::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Block {
    type Builder = BlockBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Block(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BlockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .header(self.header())
            .uncles(self.uncles())
            .transactions(self.transactions())
            .proposals(self.proposals())
    }
}
impl Block {
    pub const NAME: &'static str = "Block";
    pub fn as_reader(&self) -> BlockReader<'_> {
        BlockReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 4;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> Header {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Header::new_unchecked(self.0.slice(start, end))
    }
    pub fn uncles(&self) -> UncleBlockVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        UncleBlockVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn transactions(&self) -> TransactionVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        TransactionVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn proposals(&self) -> ProposalShortIdVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        if count == 4 {
            ProposalShortIdVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[3 + 1]) as usize;
            ProposalShortIdVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BlockReader<'r> {
    type Entity = Block;
    fn to_entity(&self) -> Self::Entity {
        Block::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BlockReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 4;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=4]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        HeaderReader::verify(&slice[offsets[0]..offsets[1]])?;
        UncleBlockVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        TransactionVecReader::verify(&slice[offsets[2]..offsets[3]])?;
        ProposalShortIdVecReader::verify(&slice[offsets[3]..offsets[4]])?;
        Ok(())
    }
}
impl<'r> BlockReader<'r> {
    pub const NAME: &'r str = "BlockReader";
    pub const FIELD_COUNT: usize = 4;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> HeaderReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        HeaderReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn uncles(&self) -> UncleBlockVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        UncleBlockVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn transactions(&self) -> TransactionVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        TransactionVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proposals(&self) -> ProposalShortIdVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        if count == 4 {
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[3 + 1]) as usize;
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for BlockBuilder {
    type Entity = Block;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * 4;
        len_header
            + self.header.as_slice().len()
            + self.uncles.as_slice().len()
            + self.transactions.as_slice().len()
            + self.proposals.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.header.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.uncles.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transactions.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.proposals.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.header.as_slice())?;
        writer.write_all(self.uncles.as_slice())?;
        writer.write_all(self.transactions.as_slice())?;
        writer.write_all(self.proposals.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Block::new_unchecked(inner.into())
    }
}
impl BlockBuilder {
    pub const NAME: &'static str = "BlockBuilder";
    pub fn header(mut self, v: Header) -> Self {
        self.header = v;
        self
    }
    pub fn uncles(mut self, v: UncleBlockVec) -> Self {
        self.uncles = v;
        self
    }
    pub fn transactions(mut self, v: TransactionVec) -> Self {
        self.transactions = v;
        self
    }
    pub fn proposals(mut self, v: ProposalShortIdVec) -> Self {
        self.proposals = v;
        self
    }
}
#[derive(Clone)]
pub struct HeaderView(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct HeaderViewReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for HeaderView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for HeaderViewReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for HeaderView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "data", self.data())?;
        write!(f, ", {}: {}", "hash", self.hash())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for HeaderViewReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "data", self.data())?;
        write!(f, ", {}: {}", "hash", self.hash())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct HeaderViewBuilder {
    pub(crate) data: Header,
    pub(crate) hash: Byte32,
}
impl ::std::default::Default for HeaderView {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            104, 1, 0, 0, 12, 0, 0, 0, 72, 1, 0, 0, 60, 1, 0, 0, 12, 0, 0, 0, 36, 1, 0, 0, 24, 1,
            0, 0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0, 0, 0, 96, 0, 0, 0, 104, 0, 0, 0, 136, 0, 0, 0,
            168, 0, 0, 0, 200, 0, 0, 0, 232, 0, 0, 0, 8, 1, 0, 0, 12, 1, 0, 0, 20, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 12,
            0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        HeaderView::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for HeaderView {
    type Builder = HeaderViewBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        HeaderView(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        HeaderViewReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().data(self.data()).hash(self.hash())
    }
}
impl HeaderView {
    pub const NAME: &'static str = "HeaderView";
    pub fn as_reader(&self) -> HeaderViewReader<'_> {
        HeaderViewReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn data(&self) -> Header {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Header::new_unchecked(self.0.slice(start, end))
    }
    pub fn hash(&self) -> Byte32 {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for HeaderViewReader<'r> {
    type Entity = HeaderView;
    fn to_entity(&self) -> Self::Entity {
        HeaderView::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        HeaderViewReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        HeaderReader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> HeaderViewReader<'r> {
    pub const NAME: &'r str = "HeaderViewReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn data(&self) -> HeaderReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        HeaderReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn hash(&self) -> Byte32Reader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32Reader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32Reader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for HeaderViewBuilder {
    type Entity = HeaderView;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.data.as_slice().len() + self.hash.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.data.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.hash.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.data.as_slice())?;
        writer.write_all(self.hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        HeaderView::new_unchecked(inner.into())
    }
}
impl HeaderViewBuilder {
    pub const NAME: &'static str = "HeaderViewBuilder";
    pub fn data(mut self, v: Header) -> Self {
        self.data = v;
        self
    }
    pub fn hash(mut self, v: Byte32) -> Self {
        self.hash = v;
        self
    }
}
#[derive(Clone)]
pub struct UncleBlockVecView(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct UncleBlockVecViewReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for UncleBlockVecView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for UncleBlockVecViewReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for UncleBlockVecView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "data", self.data())?;
        write!(f, ", {}: {}", "hashes", self.hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for UncleBlockVecViewReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "data", self.data())?;
        write!(f, ", {}: {}", "hashes", self.hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct UncleBlockVecViewBuilder {
    pub(crate) data: UncleBlockVec,
    pub(crate) hashes: Byte32Vec,
}
impl ::std::default::Default for UncleBlockVecView {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            20, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
        ];
        UncleBlockVecView::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for UncleBlockVecView {
    type Builder = UncleBlockVecViewBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        UncleBlockVecView(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        UncleBlockVecViewReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().data(self.data()).hashes(self.hashes())
    }
}
impl UncleBlockVecView {
    pub const NAME: &'static str = "UncleBlockVecView";
    pub fn as_reader(&self) -> UncleBlockVecViewReader<'_> {
        UncleBlockVecViewReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn data(&self) -> UncleBlockVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        UncleBlockVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn hashes(&self) -> Byte32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for UncleBlockVecViewReader<'r> {
    type Entity = UncleBlockVecView;
    fn to_entity(&self) -> Self::Entity {
        UncleBlockVecView::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        UncleBlockVecViewReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        UncleBlockVecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32VecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> UncleBlockVecViewReader<'r> {
    pub const NAME: &'r str = "UncleBlockVecViewReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn data(&self) -> UncleBlockVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        UncleBlockVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn hashes(&self) -> Byte32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for UncleBlockVecViewBuilder {
    type Entity = UncleBlockVecView;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.data.as_slice().len() + self.hashes.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.data.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.hashes.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.data.as_slice())?;
        writer.write_all(self.hashes.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        UncleBlockVecView::new_unchecked(inner.into())
    }
}
impl UncleBlockVecViewBuilder {
    pub const NAME: &'static str = "UncleBlockVecViewBuilder";
    pub fn data(mut self, v: UncleBlockVec) -> Self {
        self.data = v;
        self
    }
    pub fn hashes(mut self, v: Byte32Vec) -> Self {
        self.hashes = v;
        self
    }
}
#[derive(Clone)]
pub struct BlockBodyView(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BlockBodyViewReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for BlockBodyView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BlockBodyViewReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for BlockBodyView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "data", self.data())?;
        write!(f, ", {}: {}", "tx_hashes", self.tx_hashes())?;
        write!(f, ", {}: {}", "tx_witness_hashes", self.tx_witness_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for BlockBodyViewReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "data", self.data())?;
        write!(f, ", {}: {}", "tx_hashes", self.tx_hashes())?;
        write!(f, ", {}: {}", "tx_witness_hashes", self.tx_witness_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct BlockBodyViewBuilder {
    pub(crate) data: TransactionVec,
    pub(crate) tx_hashes: Byte32Vec,
    pub(crate) tx_witness_hashes: Byte32Vec,
}
impl ::std::default::Default for BlockBodyView {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            28, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        BlockBodyView::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for BlockBodyView {
    type Builder = BlockBodyViewBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        BlockBodyView(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BlockBodyViewReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .data(self.data())
            .tx_hashes(self.tx_hashes())
            .tx_witness_hashes(self.tx_witness_hashes())
    }
}
impl BlockBodyView {
    pub const NAME: &'static str = "BlockBodyView";
    pub fn as_reader(&self) -> BlockBodyViewReader<'_> {
        BlockBodyViewReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn data(&self) -> TransactionVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        TransactionVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn tx_hashes(&self) -> Byte32Vec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32Vec::new_unchecked(self.0.slice(start, end))
    }
    pub fn tx_witness_hashes(&self) -> Byte32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            Byte32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            Byte32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BlockBodyViewReader<'r> {
    type Entity = BlockBodyView;
    fn to_entity(&self) -> Self::Entity {
        BlockBodyView::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BlockBodyViewReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        TransactionVecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32VecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Byte32VecReader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> BlockBodyViewReader<'r> {
    pub const NAME: &'r str = "BlockBodyViewReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn data(&self) -> TransactionVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        TransactionVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn tx_hashes(&self) -> Byte32VecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn tx_witness_hashes(&self) -> Byte32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            Byte32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for BlockBodyViewBuilder {
    type Entity = BlockBodyView;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header
            + self.data.as_slice().len()
            + self.tx_hashes.as_slice().len()
            + self.tx_witness_hashes.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.data.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.tx_hashes.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.tx_witness_hashes.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.data.as_slice())?;
        writer.write_all(self.tx_hashes.as_slice())?;
        writer.write_all(self.tx_witness_hashes.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        BlockBodyView::new_unchecked(inner.into())
    }
}
impl BlockBodyViewBuilder {
    pub const NAME: &'static str = "BlockBodyViewBuilder";
    pub fn data(mut self, v: TransactionVec) -> Self {
        self.data = v;
        self
    }
    pub fn tx_hashes(mut self, v: Byte32Vec) -> Self {
        self.tx_hashes = v;
        self
    }
    pub fn tx_witness_hashes(mut self, v: Byte32Vec) -> Self {
        self.tx_witness_hashes = v;
        self
    }
}
#[derive(Clone)]
pub struct BlockExt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BlockExtReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for BlockExt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BlockExtReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for BlockExt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "received_at", self.received_at())?;
        write!(f, ", {}: {}", "total_difficulty", self.total_difficulty())?;
        write!(
            f,
            ", {}: {}",
            "total_uncles_count",
            self.total_uncles_count()
        )?;
        write!(f, ", {}: {}", "verified", self.verified())?;
        write!(f, ", {}: {}", "txs_fees", self.txs_fees())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 5 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for BlockExtReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "received_at", self.received_at())?;
        write!(f, ", {}: {}", "total_difficulty", self.total_difficulty())?;
        write!(
            f,
            ", {}: {}",
            "total_uncles_count",
            self.total_uncles_count()
        )?;
        write!(f, ", {}: {}", "verified", self.verified())?;
        write!(f, ", {}: {}", "txs_fees", self.txs_fees())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 5 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct BlockExtBuilder {
    pub(crate) received_at: Uint64,
    pub(crate) total_difficulty: Byte32,
    pub(crate) total_uncles_count: Uint64,
    pub(crate) verified: BoolOpt,
    pub(crate) txs_fees: Uint64Vec,
}
impl ::std::default::Default for BlockExt {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            76, 0, 0, 0, 24, 0, 0, 0, 32, 0, 0, 0, 64, 0, 0, 0, 72, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        BlockExt::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for BlockExt {
    type Builder = BlockExtBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        BlockExt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BlockExtReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .received_at(self.received_at())
            .total_difficulty(self.total_difficulty())
            .total_uncles_count(self.total_uncles_count())
            .verified(self.verified())
            .txs_fees(self.txs_fees())
    }
}
impl BlockExt {
    pub const NAME: &'static str = "BlockExt";
    pub fn as_reader(&self) -> BlockExtReader<'_> {
        BlockExtReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 5;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn received_at(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn total_difficulty(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn total_uncles_count(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn verified(&self) -> BoolOpt {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        BoolOpt::new_unchecked(self.0.slice(start, end))
    }
    pub fn txs_fees(&self) -> Uint64Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        if count == 5 {
            Uint64Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[4 + 1]) as usize;
            Uint64Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BlockExtReader<'r> {
    type Entity = BlockExt;
    fn to_entity(&self) -> Self::Entity {
        BlockExt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BlockExtReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 5;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=5]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Uint64Reader::verify(&slice[offsets[2]..offsets[3]])?;
        BoolOptReader::verify(&slice[offsets[3]..offsets[4]])?;
        Uint64VecReader::verify(&slice[offsets[4]..offsets[5]])?;
        Ok(())
    }
}
impl<'r> BlockExtReader<'r> {
    pub const NAME: &'r str = "BlockExtReader";
    pub const FIELD_COUNT: usize = 5;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn received_at(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn total_difficulty(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn total_uncles_count(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn verified(&self) -> BoolOptReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        BoolOptReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn txs_fees(&self) -> Uint64VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        if count == 5 {
            Uint64VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[4 + 1]) as usize;
            Uint64VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for BlockExtBuilder {
    type Entity = BlockExt;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 5 * 4;
        len_header
            + self.received_at.as_slice().len()
            + self.total_difficulty.as_slice().len()
            + self.total_uncles_count.as_slice().len()
            + self.verified.as_slice().len()
            + self.txs_fees.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 5 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.received_at.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.total_difficulty.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.total_uncles_count.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.verified.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.txs_fees.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.received_at.as_slice())?;
        writer.write_all(self.total_difficulty.as_slice())?;
        writer.write_all(self.total_uncles_count.as_slice())?;
        writer.write_all(self.verified.as_slice())?;
        writer.write_all(self.txs_fees.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        BlockExt::new_unchecked(inner.into())
    }
}
impl BlockExtBuilder {
    pub const NAME: &'static str = "BlockExtBuilder";
    pub fn received_at(mut self, v: Uint64) -> Self {
        self.received_at = v;
        self
    }
    pub fn total_difficulty(mut self, v: Byte32) -> Self {
        self.total_difficulty = v;
        self
    }
    pub fn total_uncles_count(mut self, v: Uint64) -> Self {
        self.total_uncles_count = v;
        self
    }
    pub fn verified(mut self, v: BoolOpt) -> Self {
        self.verified = v;
        self
    }
    pub fn txs_fees(mut self, v: Uint64Vec) -> Self {
        self.txs_fees = v;
        self
    }
}
#[derive(Clone)]
pub struct EpochExt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct EpochExtReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for EpochExt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for EpochExtReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for EpochExt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "number", self.number())?;
        write!(f, ", {}: {}", "block_reward", self.block_reward())?;
        write!(f, ", {}: {}", "remainder_reward", self.remainder_reward())?;
        write!(f, ", {}: {}", "start_number", self.start_number())?;
        write!(f, ", {}: {}", "length", self.length())?;
        write!(f, ", {}: {}", "difficulty", self.difficulty())?;
        write!(
            f,
            ", {}: {}",
            "last_block_hash_in_previous_epoch",
            self.last_block_hash_in_previous_epoch()
        )?;
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for EpochExtReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "number", self.number())?;
        write!(f, ", {}: {}", "block_reward", self.block_reward())?;
        write!(f, ", {}: {}", "remainder_reward", self.remainder_reward())?;
        write!(f, ", {}: {}", "start_number", self.start_number())?;
        write!(f, ", {}: {}", "length", self.length())?;
        write!(f, ", {}: {}", "difficulty", self.difficulty())?;
        write!(
            f,
            ", {}: {}",
            "last_block_hash_in_previous_epoch",
            self.last_block_hash_in_previous_epoch()
        )?;
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct EpochExtBuilder {
    pub(crate) number: Uint64,
    pub(crate) block_reward: Uint64,
    pub(crate) remainder_reward: Uint64,
    pub(crate) start_number: Uint64,
    pub(crate) length: Uint64,
    pub(crate) difficulty: Byte32,
    pub(crate) last_block_hash_in_previous_epoch: Byte32,
}
impl molecule::prelude::Entity for EpochExt {
    type Builder = EpochExtBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        EpochExt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        EpochExtReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .number(self.number())
            .block_reward(self.block_reward())
            .remainder_reward(self.remainder_reward())
            .start_number(self.start_number())
            .length(self.length())
            .difficulty(self.difficulty())
            .last_block_hash_in_previous_epoch(self.last_block_hash_in_previous_epoch())
    }
}
impl ::std::default::Default for EpochExt {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        EpochExt::new_unchecked(v.into())
    }
}
impl EpochExt {
    pub const NAME: &'static str = "EpochExt";
    pub fn as_reader(&self) -> EpochExtReader<'_> {
        EpochExtReader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 104;
    pub const FIELD_COUNT: usize = 7;
    pub const FIELDS_SIZE: [usize; 7] = [8, 8, 8, 8, 8, 32, 32];
    pub fn number(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(0, 8))
    }
    pub fn block_reward(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(8, 16))
    }
    pub fn remainder_reward(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(16, 24))
    }
    pub fn start_number(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(24, 32))
    }
    pub fn length(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(32, 40))
    }
    pub fn difficulty(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(40, 72))
    }
    pub fn last_block_hash_in_previous_epoch(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(72, 104))
    }
}
impl<'r> molecule::prelude::Reader<'r> for EpochExtReader<'r> {
    type Entity = EpochExt;
    fn to_entity(&self) -> Self::Entity {
        EpochExt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        EpochExtReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 104 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 104, slice.len());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[0..8])?;
        Uint64Reader::verify(&slice[8..16])?;
        Uint64Reader::verify(&slice[16..24])?;
        Uint64Reader::verify(&slice[24..32])?;
        Uint64Reader::verify(&slice[32..40])?;
        Byte32Reader::verify(&slice[40..72])?;
        Byte32Reader::verify(&slice[72..104])?;
        Ok(())
    }
}
impl<'r> EpochExtReader<'r> {
    pub const NAME: &'r str = "EpochExtReader";
    pub const TOTAL_SIZE: usize = 104;
    pub const FIELD_COUNT: usize = 7;
    pub const FIELDS_SIZE: [usize; 7] = [8, 8, 8, 8, 8, 32, 32];
    pub fn number(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[0..8])
    }
    pub fn block_reward(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[8..16])
    }
    pub fn remainder_reward(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[16..24])
    }
    pub fn start_number(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[24..32])
    }
    pub fn length(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[32..40])
    }
    pub fn difficulty(&self) -> Byte32Reader<'_> {
        Byte32Reader::new_unchecked(&self.as_slice()[40..72])
    }
    pub fn last_block_hash_in_previous_epoch(&self) -> Byte32Reader<'_> {
        Byte32Reader::new_unchecked(&self.as_slice()[72..104])
    }
}
impl molecule::prelude::Builder for EpochExtBuilder {
    type Entity = EpochExt;
    fn expected_length(&self) -> usize {
        104
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(self.number.as_slice())?;
        writer.write_all(self.block_reward.as_slice())?;
        writer.write_all(self.remainder_reward.as_slice())?;
        writer.write_all(self.start_number.as_slice())?;
        writer.write_all(self.length.as_slice())?;
        writer.write_all(self.difficulty.as_slice())?;
        writer.write_all(self.last_block_hash_in_previous_epoch.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        EpochExt::new_unchecked(inner.into())
    }
}
impl EpochExtBuilder {
    pub const NAME: &'static str = "EpochExtBuilder";
    pub fn number(mut self, v: Uint64) -> Self {
        self.number = v;
        self
    }
    pub fn block_reward(mut self, v: Uint64) -> Self {
        self.block_reward = v;
        self
    }
    pub fn remainder_reward(mut self, v: Uint64) -> Self {
        self.remainder_reward = v;
        self
    }
    pub fn start_number(mut self, v: Uint64) -> Self {
        self.start_number = v;
        self
    }
    pub fn length(mut self, v: Uint64) -> Self {
        self.length = v;
        self
    }
    pub fn difficulty(mut self, v: Byte32) -> Self {
        self.difficulty = v;
        self
    }
    pub fn last_block_hash_in_previous_epoch(mut self, v: Byte32) -> Self {
        self.last_block_hash_in_previous_epoch = v;
        self
    }
}
#[derive(Clone)]
pub struct TransactionInfo(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct TransactionInfoReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for TransactionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for TransactionInfoReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for TransactionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "index", self.index())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "block_epoch", self.block_epoch())?;
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for TransactionInfoReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "index", self.index())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "block_epoch", self.block_epoch())?;
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct TransactionInfoBuilder {
    pub(crate) index: Uint32,
    pub(crate) block_hash: Byte32,
    pub(crate) block_number: Uint64,
    pub(crate) block_epoch: Uint64,
}
impl molecule::prelude::Entity for TransactionInfo {
    type Builder = TransactionInfoBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        TransactionInfo(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        TransactionInfoReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .index(self.index())
            .block_hash(self.block_hash())
            .block_number(self.block_number())
            .block_epoch(self.block_epoch())
    }
}
impl ::std::default::Default for TransactionInfo {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        TransactionInfo::new_unchecked(v.into())
    }
}
impl TransactionInfo {
    pub const NAME: &'static str = "TransactionInfo";
    pub fn as_reader(&self) -> TransactionInfoReader<'_> {
        TransactionInfoReader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 52;
    pub const FIELD_COUNT: usize = 4;
    pub const FIELDS_SIZE: [usize; 4] = [4, 32, 8, 8];
    pub fn index(&self) -> Uint32 {
        Uint32::new_unchecked(self.0.slice(0, 4))
    }
    pub fn block_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(4, 36))
    }
    pub fn block_number(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(36, 44))
    }
    pub fn block_epoch(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(44, 52))
    }
}
impl<'r> molecule::prelude::Reader<'r> for TransactionInfoReader<'r> {
    type Entity = TransactionInfo;
    fn to_entity(&self) -> Self::Entity {
        TransactionInfo::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        TransactionInfoReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 52 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 52, slice.len());
            Err(err)?;
        }
        Uint32Reader::verify(&slice[0..4])?;
        Byte32Reader::verify(&slice[4..36])?;
        Uint64Reader::verify(&slice[36..44])?;
        Uint64Reader::verify(&slice[44..52])?;
        Ok(())
    }
}
impl<'r> TransactionInfoReader<'r> {
    pub const NAME: &'r str = "TransactionInfoReader";
    pub const TOTAL_SIZE: usize = 52;
    pub const FIELD_COUNT: usize = 4;
    pub const FIELDS_SIZE: [usize; 4] = [4, 32, 8, 8];
    pub fn index(&self) -> Uint32Reader<'_> {
        Uint32Reader::new_unchecked(&self.as_slice()[0..4])
    }
    pub fn block_hash(&self) -> Byte32Reader<'_> {
        Byte32Reader::new_unchecked(&self.as_slice()[4..36])
    }
    pub fn block_number(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[36..44])
    }
    pub fn block_epoch(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[44..52])
    }
}
impl molecule::prelude::Builder for TransactionInfoBuilder {
    type Entity = TransactionInfo;
    fn expected_length(&self) -> usize {
        52
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(self.index.as_slice())?;
        writer.write_all(self.block_hash.as_slice())?;
        writer.write_all(self.block_number.as_slice())?;
        writer.write_all(self.block_epoch.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        TransactionInfo::new_unchecked(inner.into())
    }
}
impl TransactionInfoBuilder {
    pub const NAME: &'static str = "TransactionInfoBuilder";
    pub fn index(mut self, v: Uint32) -> Self {
        self.index = v;
        self
    }
    pub fn block_hash(mut self, v: Byte32) -> Self {
        self.block_hash = v;
        self
    }
    pub fn block_number(mut self, v: Uint64) -> Self {
        self.block_number = v;
        self
    }
    pub fn block_epoch(mut self, v: Uint64) -> Self {
        self.block_epoch = v;
        self
    }
}
#[derive(Clone)]
pub struct TransactionMeta(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct TransactionMetaReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for TransactionMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for TransactionMetaReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for TransactionMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "epoch_number", self.epoch_number())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "len", self.len())?;
        write!(f, ", {}: {}", "bits", self.bits())?;
        write!(f, ", {}: {}", "cellbase", self.cellbase())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 6 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for TransactionMetaReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "epoch_number", self.epoch_number())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "len", self.len())?;
        write!(f, ", {}: {}", "bits", self.bits())?;
        write!(f, ", {}: {}", "cellbase", self.cellbase())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 6 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct TransactionMetaBuilder {
    pub(crate) block_number: Uint64,
    pub(crate) epoch_number: Uint64,
    pub(crate) block_hash: Byte32,
    pub(crate) len: Uint32,
    pub(crate) bits: Bytes,
    pub(crate) cellbase: Bool,
}
impl ::std::default::Default for TransactionMeta {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            85, 0, 0, 0, 28, 0, 0, 0, 36, 0, 0, 0, 44, 0, 0, 0, 76, 0, 0, 0, 80, 0, 0, 0, 84, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        TransactionMeta::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for TransactionMeta {
    type Builder = TransactionMetaBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        TransactionMeta(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        TransactionMetaReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .block_number(self.block_number())
            .epoch_number(self.epoch_number())
            .block_hash(self.block_hash())
            .len(self.len())
            .bits(self.bits())
            .cellbase(self.cellbase())
    }
}
impl TransactionMeta {
    pub const NAME: &'static str = "TransactionMeta";
    pub fn as_reader(&self) -> TransactionMetaReader<'_> {
        TransactionMetaReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 6;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn epoch_number(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn block_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn len(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn bits(&self) -> Bytes {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        let end = u32::from_le(offsets[4 + 1]) as usize;
        Bytes::new_unchecked(self.0.slice(start, end))
    }
    pub fn cellbase(&self) -> Bool {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[5]) as usize;
        if count == 6 {
            Bool::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[5 + 1]) as usize;
            Bool::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for TransactionMetaReader<'r> {
    type Entity = TransactionMeta;
    fn to_entity(&self) -> Self::Entity {
        TransactionMeta::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        TransactionMetaReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 6;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=6]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Uint64Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Byte32Reader::verify(&slice[offsets[2]..offsets[3]])?;
        Uint32Reader::verify(&slice[offsets[3]..offsets[4]])?;
        BytesReader::verify(&slice[offsets[4]..offsets[5]])?;
        BoolReader::verify(&slice[offsets[5]..offsets[6]])?;
        Ok(())
    }
}
impl<'r> TransactionMetaReader<'r> {
    pub const NAME: &'r str = "TransactionMetaReader";
    pub const FIELD_COUNT: usize = 6;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn epoch_number(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn block_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn len(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn bits(&self) -> BytesReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        let end = u32::from_le(offsets[4 + 1]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn cellbase(&self) -> BoolReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[5]) as usize;
        if count == 6 {
            BoolReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[5 + 1]) as usize;
            BoolReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for TransactionMetaBuilder {
    type Entity = TransactionMeta;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 6 * 4;
        len_header
            + self.block_number.as_slice().len()
            + self.epoch_number.as_slice().len()
            + self.block_hash.as_slice().len()
            + self.len.as_slice().len()
            + self.bits.as_slice().len()
            + self.cellbase.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 6 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_number.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.epoch_number.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.len.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.bits.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.cellbase.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_number.as_slice())?;
        writer.write_all(self.epoch_number.as_slice())?;
        writer.write_all(self.block_hash.as_slice())?;
        writer.write_all(self.len.as_slice())?;
        writer.write_all(self.bits.as_slice())?;
        writer.write_all(self.cellbase.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        TransactionMeta::new_unchecked(inner.into())
    }
}
impl TransactionMetaBuilder {
    pub const NAME: &'static str = "TransactionMetaBuilder";
    pub fn block_number(mut self, v: Uint64) -> Self {
        self.block_number = v;
        self
    }
    pub fn epoch_number(mut self, v: Uint64) -> Self {
        self.epoch_number = v;
        self
    }
    pub fn block_hash(mut self, v: Byte32) -> Self {
        self.block_hash = v;
        self
    }
    pub fn len(mut self, v: Uint32) -> Self {
        self.len = v;
        self
    }
    pub fn bits(mut self, v: Bytes) -> Self {
        self.bits = v;
        self
    }
    pub fn cellbase(mut self, v: Bool) -> Self {
        self.cellbase = v;
        self
    }
}
#[derive(Clone)]
pub struct CellMeta(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CellMetaReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CellMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CellMetaReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CellMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "capacity", self.capacity())?;
        write!(f, ", {}: {}", "data_hash", self.data_hash())?;
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for CellMetaReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "capacity", self.capacity())?;
        write!(f, ", {}: {}", "data_hash", self.data_hash())?;
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct CellMetaBuilder {
    pub(crate) capacity: Uint64,
    pub(crate) data_hash: Byte32,
}
impl molecule::prelude::Entity for CellMeta {
    type Builder = CellMetaBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CellMeta(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CellMetaReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .capacity(self.capacity())
            .data_hash(self.data_hash())
    }
}
impl ::std::default::Default for CellMeta {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        CellMeta::new_unchecked(v.into())
    }
}
impl CellMeta {
    pub const NAME: &'static str = "CellMeta";
    pub fn as_reader(&self) -> CellMetaReader<'_> {
        CellMetaReader::new_unchecked(self.as_slice())
    }
    pub const TOTAL_SIZE: usize = 40;
    pub const FIELD_COUNT: usize = 2;
    pub const FIELDS_SIZE: [usize; 2] = [8, 32];
    pub fn capacity(&self) -> Uint64 {
        Uint64::new_unchecked(self.0.slice(0, 8))
    }
    pub fn data_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(8, 40))
    }
}
impl<'r> molecule::prelude::Reader<'r> for CellMetaReader<'r> {
    type Entity = CellMeta;
    fn to_entity(&self) -> Self::Entity {
        CellMeta::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CellMetaReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() != 40 {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), 40, slice.len());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[0..8])?;
        Byte32Reader::verify(&slice[8..40])?;
        Ok(())
    }
}
impl<'r> CellMetaReader<'r> {
    pub const NAME: &'r str = "CellMetaReader";
    pub const TOTAL_SIZE: usize = 40;
    pub const FIELD_COUNT: usize = 2;
    pub const FIELDS_SIZE: [usize; 2] = [8, 32];
    pub fn capacity(&self) -> Uint64Reader<'_> {
        Uint64Reader::new_unchecked(&self.as_slice()[0..8])
    }
    pub fn data_hash(&self) -> Byte32Reader<'_> {
        Byte32Reader::new_unchecked(&self.as_slice()[8..40])
    }
}
impl molecule::prelude::Builder for CellMetaBuilder {
    type Entity = CellMeta;
    fn expected_length(&self) -> usize {
        40
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        writer.write_all(self.capacity.as_slice())?;
        writer.write_all(self.data_hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CellMeta::new_unchecked(inner.into())
    }
}
impl CellMetaBuilder {
    pub const NAME: &'static str = "CellMetaBuilder";
    pub fn capacity(mut self, v: Uint64) -> Self {
        self.capacity = v;
        self
    }
    pub fn data_hash(mut self, v: Byte32) -> Self {
        self.data_hash = v;
        self
    }
}
#[derive(Clone)]
pub struct TransactionPoint(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct TransactionPointReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for TransactionPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for TransactionPointReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for TransactionPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "tx_hash", self.tx_hash())?;
        write!(f, ", {}: {}", "index", self.index())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for TransactionPointReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "tx_hash", self.tx_hash())?;
        write!(f, ", {}: {}", "index", self.index())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct TransactionPointBuilder {
    pub(crate) block_number: Uint64,
    pub(crate) tx_hash: Byte32,
    pub(crate) index: Uint32,
}
impl ::std::default::Default for TransactionPoint {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            60, 0, 0, 0, 16, 0, 0, 0, 24, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        TransactionPoint::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for TransactionPoint {
    type Builder = TransactionPointBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        TransactionPoint(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        TransactionPointReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .block_number(self.block_number())
            .tx_hash(self.tx_hash())
            .index(self.index())
    }
}
impl TransactionPoint {
    pub const NAME: &'static str = "TransactionPoint";
    pub fn as_reader(&self) -> TransactionPointReader<'_> {
        TransactionPointReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn tx_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn index(&self) -> Uint32 {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            Uint32::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            Uint32::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for TransactionPointReader<'r> {
    type Entity = TransactionPoint;
    fn to_entity(&self) -> Self::Entity {
        TransactionPoint::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        TransactionPointReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Uint32Reader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> TransactionPointReader<'r> {
    pub const NAME: &'r str = "TransactionPointReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn tx_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn index(&self) -> Uint32Reader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            Uint32Reader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            Uint32Reader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for TransactionPointBuilder {
    type Entity = TransactionPoint;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header
            + self.block_number.as_slice().len()
            + self.tx_hash.as_slice().len()
            + self.index.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_number.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.tx_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.index.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_number.as_slice())?;
        writer.write_all(self.tx_hash.as_slice())?;
        writer.write_all(self.index.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        TransactionPoint::new_unchecked(inner.into())
    }
}
impl TransactionPointBuilder {
    pub const NAME: &'static str = "TransactionPointBuilder";
    pub fn block_number(mut self, v: Uint64) -> Self {
        self.block_number = v;
        self
    }
    pub fn tx_hash(mut self, v: Byte32) -> Self {
        self.tx_hash = v;
        self
    }
    pub fn index(mut self, v: Uint32) -> Self {
        self.index = v;
        self
    }
}
#[derive(Clone)]
pub struct TransactionPointOpt(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct TransactionPointOptReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for TransactionPointOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for TransactionPointOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for TransactionPointOpt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for TransactionPointOptReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        {
            if let Some(v) = self.to_opt() {
                write!(f, "{}(Some({}))", Self::NAME, v)
            } else {
                write!(f, "{}(None)", Self::NAME)
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct TransactionPointOptBuilder(pub(crate) Option<TransactionPoint>);
impl molecule::prelude::Entity for TransactionPointOpt {
    type Builder = TransactionPointOptBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        TransactionPointOpt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        TransactionPointOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
impl ::std::default::Default for TransactionPointOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        TransactionPointOpt::new_unchecked(v.into())
    }
}
impl TransactionPointOpt {
    pub const NAME: &'static str = "TransactionPointOpt";
    pub fn as_reader(&self) -> TransactionPointOptReader<'_> {
        TransactionPointOptReader::new_unchecked(self.as_slice())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<TransactionPoint> {
        if self.is_none() {
            None
        } else {
            Some(TransactionPoint::new_unchecked(self.0.clone()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for TransactionPointOptReader<'r> {
    type Entity = TransactionPointOpt;
    fn to_entity(&self) -> Self::Entity {
        TransactionPointOpt::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        TransactionPointOptReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            TransactionPointReader::verify(&slice[..])?;
        }
        Ok(())
    }
}
impl<'r> TransactionPointOptReader<'r> {
    pub const NAME: &'r str = "TransactionPointOptReader";
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<TransactionPointReader<'_>> {
        if self.is_none() {
            None
        } else {
            Some(TransactionPointReader::new_unchecked(self.as_slice()))
        }
    }
}
impl molecule::prelude::Builder for TransactionPointOptBuilder {
    type Entity = TransactionPointOpt;
    fn expected_length(&self) -> usize {
        if let Some(ref inner) = self.0 {
            inner.as_slice().len()
        } else {
            0
        }
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        if let Some(ref inner) = self.0 {
            writer.write_all(inner.as_slice())
        } else {
            Ok(())
        }
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        TransactionPointOpt::new_unchecked(inner.into())
    }
}
impl TransactionPointOptBuilder {
    pub const NAME: &'static str = "TransactionPointOptBuilder";
    pub fn set(mut self, v: Option<TransactionPoint>) -> Self {
        self.0 = v;
        self
    }
}
#[derive(Clone)]
pub struct LockHashCellOutput(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct LockHashCellOutputReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for LockHashCellOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for LockHashCellOutputReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for LockHashCellOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "lock_hash", self.lock_hash())?;
        write!(f, ", {}: {}", "cell_output", self.cell_output())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for LockHashCellOutputReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "lock_hash", self.lock_hash())?;
        write!(f, ", {}: {}", "cell_output", self.cell_output())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct LockHashCellOutputBuilder {
    pub(crate) block_number: Uint64,
    pub(crate) lock_hash: Byte32,
    pub(crate) cell_output: CellOutputOpt,
}
impl ::std::default::Default for LockHashCellOutput {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            56, 0, 0, 0, 16, 0, 0, 0, 24, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        LockHashCellOutput::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for LockHashCellOutput {
    type Builder = LockHashCellOutputBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        LockHashCellOutput(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        LockHashCellOutputReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .block_number(self.block_number())
            .lock_hash(self.lock_hash())
            .cell_output(self.cell_output())
    }
}
impl LockHashCellOutput {
    pub const NAME: &'static str = "LockHashCellOutput";
    pub fn as_reader(&self) -> LockHashCellOutputReader<'_> {
        LockHashCellOutputReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn lock_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn cell_output(&self) -> CellOutputOpt {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            CellOutputOpt::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            CellOutputOpt::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for LockHashCellOutputReader<'r> {
    type Entity = LockHashCellOutput;
    fn to_entity(&self) -> Self::Entity {
        LockHashCellOutput::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        LockHashCellOutputReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        CellOutputOptReader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> LockHashCellOutputReader<'r> {
    pub const NAME: &'r str = "LockHashCellOutputReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn lock_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn cell_output(&self) -> CellOutputOptReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            CellOutputOptReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            CellOutputOptReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for LockHashCellOutputBuilder {
    type Entity = LockHashCellOutput;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header
            + self.block_number.as_slice().len()
            + self.lock_hash.as_slice().len()
            + self.cell_output.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_number.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.lock_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.cell_output.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_number.as_slice())?;
        writer.write_all(self.lock_hash.as_slice())?;
        writer.write_all(self.cell_output.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        LockHashCellOutput::new_unchecked(inner.into())
    }
}
impl LockHashCellOutputBuilder {
    pub const NAME: &'static str = "LockHashCellOutputBuilder";
    pub fn block_number(mut self, v: Uint64) -> Self {
        self.block_number = v;
        self
    }
    pub fn lock_hash(mut self, v: Byte32) -> Self {
        self.lock_hash = v;
        self
    }
    pub fn cell_output(mut self, v: CellOutputOpt) -> Self {
        self.cell_output = v;
        self
    }
}
#[derive(Clone)]
pub struct LockHashIndex(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct LockHashIndexReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for LockHashIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for LockHashIndexReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for LockHashIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "lock_hash", self.lock_hash())?;
        write!(f, ", {}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "cell_out_point", self.cell_out_point())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for LockHashIndexReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "lock_hash", self.lock_hash())?;
        write!(f, ", {}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "cell_out_point", self.cell_out_point())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct LockHashIndexBuilder {
    pub(crate) lock_hash: Byte32,
    pub(crate) block_number: Uint64,
    pub(crate) cell_out_point: CellOutPoint,
}
impl ::std::default::Default for LockHashIndex {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            104, 0, 0, 0, 16, 0, 0, 0, 48, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            48, 0, 0, 0, 12, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        LockHashIndex::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for LockHashIndex {
    type Builder = LockHashIndexBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        LockHashIndex(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        LockHashIndexReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .lock_hash(self.lock_hash())
            .block_number(self.block_number())
            .cell_out_point(self.cell_out_point())
    }
}
impl LockHashIndex {
    pub const NAME: &'static str = "LockHashIndex";
    pub fn as_reader(&self) -> LockHashIndexReader<'_> {
        LockHashIndexReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn lock_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn block_number(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn cell_out_point(&self) -> CellOutPoint {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            CellOutPoint::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            CellOutPoint::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for LockHashIndexReader<'r> {
    type Entity = LockHashIndex;
    fn to_entity(&self) -> Self::Entity {
        LockHashIndex::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        LockHashIndexReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Uint64Reader::verify(&slice[offsets[1]..offsets[2]])?;
        CellOutPointReader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> LockHashIndexReader<'r> {
    pub const NAME: &'r str = "LockHashIndexReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn lock_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn block_number(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn cell_out_point(&self) -> CellOutPointReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            CellOutPointReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            CellOutPointReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for LockHashIndexBuilder {
    type Entity = LockHashIndex;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header
            + self.lock_hash.as_slice().len()
            + self.block_number.as_slice().len()
            + self.cell_out_point.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.lock_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_number.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.cell_out_point.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.lock_hash.as_slice())?;
        writer.write_all(self.block_number.as_slice())?;
        writer.write_all(self.cell_out_point.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        LockHashIndex::new_unchecked(inner.into())
    }
}
impl LockHashIndexBuilder {
    pub const NAME: &'static str = "LockHashIndexBuilder";
    pub fn lock_hash(mut self, v: Byte32) -> Self {
        self.lock_hash = v;
        self
    }
    pub fn block_number(mut self, v: Uint64) -> Self {
        self.block_number = v;
        self
    }
    pub fn cell_out_point(mut self, v: CellOutPoint) -> Self {
        self.cell_out_point = v;
        self
    }
}
#[derive(Clone)]
pub struct LockHashIndexState(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct LockHashIndexStateReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for LockHashIndexState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for LockHashIndexStateReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for LockHashIndexState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for LockHashIndexStateReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_number", self.block_number())?;
        write!(f, ", {}: {}", "block_hash", self.block_hash())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct LockHashIndexStateBuilder {
    pub(crate) block_number: Uint64,
    pub(crate) block_hash: Byte32,
}
impl ::std::default::Default for LockHashIndexState {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            52, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        LockHashIndexState::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for LockHashIndexState {
    type Builder = LockHashIndexStateBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        LockHashIndexState(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        LockHashIndexStateReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .block_number(self.block_number())
            .block_hash(self.block_hash())
    }
}
impl LockHashIndexState {
    pub const NAME: &'static str = "LockHashIndexState";
    pub fn as_reader(&self) -> LockHashIndexStateReader<'_> {
        LockHashIndexStateReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn block_hash(&self) -> Byte32 {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for LockHashIndexStateReader<'r> {
    type Entity = LockHashIndexState;
    fn to_entity(&self) -> Self::Entity {
        LockHashIndexState::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        LockHashIndexStateReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> LockHashIndexStateReader<'r> {
    pub const NAME: &'r str = "LockHashIndexStateReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_number(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn block_hash(&self) -> Byte32Reader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32Reader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32Reader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for LockHashIndexStateBuilder {
    type Entity = LockHashIndexState;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.block_number.as_slice().len() + self.block_hash.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_number.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_hash.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_number.as_slice())?;
        writer.write_all(self.block_hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        LockHashIndexState::new_unchecked(inner.into())
    }
}
impl LockHashIndexStateBuilder {
    pub const NAME: &'static str = "LockHashIndexStateBuilder";
    pub fn block_number(mut self, v: Uint64) -> Self {
        self.block_number = v;
        self
    }
    pub fn block_hash(mut self, v: Byte32) -> Self {
        self.block_hash = v;
        self
    }
}
#[derive(Clone)]
pub struct RelayMessage(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RelayMessageReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RelayMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RelayMessageReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RelayMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
impl<'r> ::std::fmt::Display for RelayMessageReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
#[derive(Debug, Clone)]
pub enum RelayMessageUnion {
    CompactBlock(CompactBlock),
    RelayTransactions(RelayTransactions),
    RelayTransactionHashes(RelayTransactionHashes),
    GetRelayTransactions(GetRelayTransactions),
    GetBlockTransactions(GetBlockTransactions),
    BlockTransactions(BlockTransactions),
    GetBlockProposal(GetBlockProposal),
    BlockProposal(BlockProposal),
}
#[derive(Debug, Clone, Copy)]
pub enum RelayMessageUnionReader<'r> {
    CompactBlock(CompactBlockReader<'r>),
    RelayTransactions(RelayTransactionsReader<'r>),
    RelayTransactionHashes(RelayTransactionHashesReader<'r>),
    GetRelayTransactions(GetRelayTransactionsReader<'r>),
    GetBlockTransactions(GetBlockTransactionsReader<'r>),
    BlockTransactions(BlockTransactionsReader<'r>),
    GetBlockProposal(GetBlockProposalReader<'r>),
    BlockProposal(BlockProposalReader<'r>),
}
impl ::std::default::Default for RelayMessageUnion {
    fn default() -> Self {
        RelayMessageUnion::CompactBlock(::std::default::Default::default())
    }
}
impl ::std::fmt::Display for RelayMessageUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            RelayMessageUnion::CompactBlock(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, CompactBlock::NAME, item)
            }
            RelayMessageUnion::RelayTransactions(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, RelayTransactions::NAME, item)
            }
            RelayMessageUnion::RelayTransactionHashes(ref item) => write!(
                f,
                "{}::{}({})",
                Self::NAME,
                RelayTransactionHashes::NAME,
                item
            ),
            RelayMessageUnion::GetRelayTransactions(ref item) => write!(
                f,
                "{}::{}({})",
                Self::NAME,
                GetRelayTransactions::NAME,
                item
            ),
            RelayMessageUnion::GetBlockTransactions(ref item) => write!(
                f,
                "{}::{}({})",
                Self::NAME,
                GetBlockTransactions::NAME,
                item
            ),
            RelayMessageUnion::BlockTransactions(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, BlockTransactions::NAME, item)
            }
            RelayMessageUnion::GetBlockProposal(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, GetBlockProposal::NAME, item)
            }
            RelayMessageUnion::BlockProposal(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, BlockProposal::NAME, item)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for RelayMessageUnionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            RelayMessageUnionReader::CompactBlock(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, CompactBlock::NAME, item)
            }
            RelayMessageUnionReader::RelayTransactions(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, RelayTransactions::NAME, item)
            }
            RelayMessageUnionReader::RelayTransactionHashes(ref item) => write!(
                f,
                "{}::{}({})",
                Self::NAME,
                RelayTransactionHashes::NAME,
                item
            ),
            RelayMessageUnionReader::GetRelayTransactions(ref item) => write!(
                f,
                "{}::{}({})",
                Self::NAME,
                GetRelayTransactions::NAME,
                item
            ),
            RelayMessageUnionReader::GetBlockTransactions(ref item) => write!(
                f,
                "{}::{}({})",
                Self::NAME,
                GetBlockTransactions::NAME,
                item
            ),
            RelayMessageUnionReader::BlockTransactions(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, BlockTransactions::NAME, item)
            }
            RelayMessageUnionReader::GetBlockProposal(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, GetBlockProposal::NAME, item)
            }
            RelayMessageUnionReader::BlockProposal(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, BlockProposal::NAME, item)
            }
        }
    }
}
impl RelayMessageUnion {
    pub(crate) fn display_inner(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            RelayMessageUnion::CompactBlock(ref item) => write!(f, "{}", item),
            RelayMessageUnion::RelayTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnion::RelayTransactionHashes(ref item) => write!(f, "{}", item),
            RelayMessageUnion::GetRelayTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnion::GetBlockTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnion::BlockTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnion::GetBlockProposal(ref item) => write!(f, "{}", item),
            RelayMessageUnion::BlockProposal(ref item) => write!(f, "{}", item),
        }
    }
}
impl<'r> RelayMessageUnionReader<'r> {
    pub(crate) fn display_inner(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            RelayMessageUnionReader::CompactBlock(ref item) => write!(f, "{}", item),
            RelayMessageUnionReader::RelayTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnionReader::RelayTransactionHashes(ref item) => write!(f, "{}", item),
            RelayMessageUnionReader::GetRelayTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnionReader::GetBlockTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnionReader::BlockTransactions(ref item) => write!(f, "{}", item),
            RelayMessageUnionReader::GetBlockProposal(ref item) => write!(f, "{}", item),
            RelayMessageUnionReader::BlockProposal(ref item) => write!(f, "{}", item),
        }
    }
}
impl ::std::convert::From<CompactBlock> for RelayMessageUnion {
    fn from(item: CompactBlock) -> Self {
        RelayMessageUnion::CompactBlock(item)
    }
}
impl ::std::convert::From<RelayTransactions> for RelayMessageUnion {
    fn from(item: RelayTransactions) -> Self {
        RelayMessageUnion::RelayTransactions(item)
    }
}
impl ::std::convert::From<RelayTransactionHashes> for RelayMessageUnion {
    fn from(item: RelayTransactionHashes) -> Self {
        RelayMessageUnion::RelayTransactionHashes(item)
    }
}
impl ::std::convert::From<GetRelayTransactions> for RelayMessageUnion {
    fn from(item: GetRelayTransactions) -> Self {
        RelayMessageUnion::GetRelayTransactions(item)
    }
}
impl ::std::convert::From<GetBlockTransactions> for RelayMessageUnion {
    fn from(item: GetBlockTransactions) -> Self {
        RelayMessageUnion::GetBlockTransactions(item)
    }
}
impl ::std::convert::From<BlockTransactions> for RelayMessageUnion {
    fn from(item: BlockTransactions) -> Self {
        RelayMessageUnion::BlockTransactions(item)
    }
}
impl ::std::convert::From<GetBlockProposal> for RelayMessageUnion {
    fn from(item: GetBlockProposal) -> Self {
        RelayMessageUnion::GetBlockProposal(item)
    }
}
impl ::std::convert::From<BlockProposal> for RelayMessageUnion {
    fn from(item: BlockProposal) -> Self {
        RelayMessageUnion::BlockProposal(item)
    }
}
impl<'r> ::std::convert::From<CompactBlockReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: CompactBlockReader<'r>) -> Self {
        RelayMessageUnionReader::CompactBlock(item)
    }
}
impl<'r> ::std::convert::From<RelayTransactionsReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: RelayTransactionsReader<'r>) -> Self {
        RelayMessageUnionReader::RelayTransactions(item)
    }
}
impl<'r> ::std::convert::From<RelayTransactionHashesReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: RelayTransactionHashesReader<'r>) -> Self {
        RelayMessageUnionReader::RelayTransactionHashes(item)
    }
}
impl<'r> ::std::convert::From<GetRelayTransactionsReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: GetRelayTransactionsReader<'r>) -> Self {
        RelayMessageUnionReader::GetRelayTransactions(item)
    }
}
impl<'r> ::std::convert::From<GetBlockTransactionsReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: GetBlockTransactionsReader<'r>) -> Self {
        RelayMessageUnionReader::GetBlockTransactions(item)
    }
}
impl<'r> ::std::convert::From<BlockTransactionsReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: BlockTransactionsReader<'r>) -> Self {
        RelayMessageUnionReader::BlockTransactions(item)
    }
}
impl<'r> ::std::convert::From<GetBlockProposalReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: GetBlockProposalReader<'r>) -> Self {
        RelayMessageUnionReader::GetBlockProposal(item)
    }
}
impl<'r> ::std::convert::From<BlockProposalReader<'r>> for RelayMessageUnionReader<'r> {
    fn from(item: BlockProposalReader<'r>) -> Self {
        RelayMessageUnionReader::BlockProposal(item)
    }
}
impl RelayMessageUnion {
    pub const NAME: &'static str = "RelayMessageUnion";
    pub fn as_bytes(&self) -> molecule::bytes::Bytes {
        match self {
            RelayMessageUnion::CompactBlock(item) => item.as_bytes(),
            RelayMessageUnion::RelayTransactions(item) => item.as_bytes(),
            RelayMessageUnion::RelayTransactionHashes(item) => item.as_bytes(),
            RelayMessageUnion::GetRelayTransactions(item) => item.as_bytes(),
            RelayMessageUnion::GetBlockTransactions(item) => item.as_bytes(),
            RelayMessageUnion::BlockTransactions(item) => item.as_bytes(),
            RelayMessageUnion::GetBlockProposal(item) => item.as_bytes(),
            RelayMessageUnion::BlockProposal(item) => item.as_bytes(),
        }
    }
    pub fn as_slice(&self) -> &[u8] {
        match self {
            RelayMessageUnion::CompactBlock(item) => item.as_slice(),
            RelayMessageUnion::RelayTransactions(item) => item.as_slice(),
            RelayMessageUnion::RelayTransactionHashes(item) => item.as_slice(),
            RelayMessageUnion::GetRelayTransactions(item) => item.as_slice(),
            RelayMessageUnion::GetBlockTransactions(item) => item.as_slice(),
            RelayMessageUnion::BlockTransactions(item) => item.as_slice(),
            RelayMessageUnion::GetBlockProposal(item) => item.as_slice(),
            RelayMessageUnion::BlockProposal(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> usize {
        match self {
            RelayMessageUnion::CompactBlock(_) => 1,
            RelayMessageUnion::RelayTransactions(_) => 2,
            RelayMessageUnion::RelayTransactionHashes(_) => 3,
            RelayMessageUnion::GetRelayTransactions(_) => 4,
            RelayMessageUnion::GetBlockTransactions(_) => 5,
            RelayMessageUnion::BlockTransactions(_) => 6,
            RelayMessageUnion::GetBlockProposal(_) => 7,
            RelayMessageUnion::BlockProposal(_) => 8,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            RelayMessageUnion::CompactBlock(_) => "CompactBlock",
            RelayMessageUnion::RelayTransactions(_) => "RelayTransactions",
            RelayMessageUnion::RelayTransactionHashes(_) => "RelayTransactionHashes",
            RelayMessageUnion::GetRelayTransactions(_) => "GetRelayTransactions",
            RelayMessageUnion::GetBlockTransactions(_) => "GetBlockTransactions",
            RelayMessageUnion::BlockTransactions(_) => "BlockTransactions",
            RelayMessageUnion::GetBlockProposal(_) => "GetBlockProposal",
            RelayMessageUnion::BlockProposal(_) => "BlockProposal",
        }
    }
    pub fn as_reader(&self) -> RelayMessageUnionReader<'_> {
        match self {
            RelayMessageUnion::CompactBlock(item) => item.as_reader().into(),
            RelayMessageUnion::RelayTransactions(item) => item.as_reader().into(),
            RelayMessageUnion::RelayTransactionHashes(item) => item.as_reader().into(),
            RelayMessageUnion::GetRelayTransactions(item) => item.as_reader().into(),
            RelayMessageUnion::GetBlockTransactions(item) => item.as_reader().into(),
            RelayMessageUnion::BlockTransactions(item) => item.as_reader().into(),
            RelayMessageUnion::GetBlockProposal(item) => item.as_reader().into(),
            RelayMessageUnion::BlockProposal(item) => item.as_reader().into(),
        }
    }
}
impl<'r> RelayMessageUnionReader<'r> {
    pub const NAME: &'r str = "RelayMessageUnionReader";
    pub fn as_slice(&self) -> &[u8] {
        match self {
            RelayMessageUnionReader::CompactBlock(item) => item.as_slice(),
            RelayMessageUnionReader::RelayTransactions(item) => item.as_slice(),
            RelayMessageUnionReader::RelayTransactionHashes(item) => item.as_slice(),
            RelayMessageUnionReader::GetRelayTransactions(item) => item.as_slice(),
            RelayMessageUnionReader::GetBlockTransactions(item) => item.as_slice(),
            RelayMessageUnionReader::BlockTransactions(item) => item.as_slice(),
            RelayMessageUnionReader::GetBlockProposal(item) => item.as_slice(),
            RelayMessageUnionReader::BlockProposal(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> usize {
        match self {
            RelayMessageUnionReader::CompactBlock(_) => 1,
            RelayMessageUnionReader::RelayTransactions(_) => 2,
            RelayMessageUnionReader::RelayTransactionHashes(_) => 3,
            RelayMessageUnionReader::GetRelayTransactions(_) => 4,
            RelayMessageUnionReader::GetBlockTransactions(_) => 5,
            RelayMessageUnionReader::BlockTransactions(_) => 6,
            RelayMessageUnionReader::GetBlockProposal(_) => 7,
            RelayMessageUnionReader::BlockProposal(_) => 8,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            RelayMessageUnionReader::CompactBlock(_) => "CompactBlock",
            RelayMessageUnionReader::RelayTransactions(_) => "RelayTransactions",
            RelayMessageUnionReader::RelayTransactionHashes(_) => "RelayTransactionHashes",
            RelayMessageUnionReader::GetRelayTransactions(_) => "GetRelayTransactions",
            RelayMessageUnionReader::GetBlockTransactions(_) => "GetBlockTransactions",
            RelayMessageUnionReader::BlockTransactions(_) => "BlockTransactions",
            RelayMessageUnionReader::GetBlockProposal(_) => "GetBlockProposal",
            RelayMessageUnionReader::BlockProposal(_) => "BlockProposal",
        }
    }
}
#[derive(Debug, Default)]
pub struct RelayMessageBuilder(pub(crate) RelayMessageUnion);
impl molecule::prelude::Entity for RelayMessage {
    type Builder = RelayMessageBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RelayMessage(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RelayMessageReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_enum())
    }
}
impl ::std::default::Default for RelayMessage {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        RelayMessage::new_unchecked(v.into())
    }
}
impl RelayMessage {
    pub const NAME: &'static str = "RelayMessage";
    pub fn as_reader(&self) -> RelayMessageReader<'_> {
        RelayMessageReader::new_unchecked(self.as_slice())
    }
    pub const ITEM_COUNT: usize = 8;
    pub fn item_id(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn to_enum(&self) -> RelayMessageUnion {
        let inner = self.0.slice_from(4);
        match self.item_id() {
            1 => CompactBlock::new_unchecked(inner).into(),
            2 => RelayTransactions::new_unchecked(inner).into(),
            3 => RelayTransactionHashes::new_unchecked(inner).into(),
            4 => GetRelayTransactions::new_unchecked(inner).into(),
            5 => GetBlockTransactions::new_unchecked(inner).into(),
            6 => BlockTransactions::new_unchecked(inner).into(),
            7 => GetBlockProposal::new_unchecked(inner).into(),
            8 => BlockProposal::new_unchecked(inner).into(),
            _ => unreachable!(),
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RelayMessageReader<'r> {
    type Entity = RelayMessage;
    fn to_entity(&self) -> Self::Entity {
        RelayMessage::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RelayMessageReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, slice.len());
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let item_id = u32::from_le(ptr[0]) as usize;
        match item_id {
            1 => CompactBlockReader::verify(&slice[4..]),
            2 => RelayTransactionsReader::verify(&slice[4..]),
            3 => RelayTransactionHashesReader::verify(&slice[4..]),
            4 => GetRelayTransactionsReader::verify(&slice[4..]),
            5 => GetBlockTransactionsReader::verify(&slice[4..]),
            6 => BlockTransactionsReader::verify(&slice[4..]),
            7 => GetBlockProposalReader::verify(&slice[4..]),
            8 => BlockProposalReader::verify(&slice[4..]),
            _ => {
                let err = VerificationError::UnknownItem(Self::NAME.to_owned(), 8, item_id);
                Err(err)
            }
        }?;
        Ok(())
    }
}
impl<'r> RelayMessageReader<'r> {
    pub const NAME: &'r str = "RelayMessageReader";
    pub const ITEM_COUNT: usize = 8;
    pub fn item_id(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn to_enum(&self) -> RelayMessageUnionReader<'_> {
        let inner = &self.as_slice()[4..];
        match self.item_id() {
            1 => CompactBlockReader::new_unchecked(inner).into(),
            2 => RelayTransactionsReader::new_unchecked(inner).into(),
            3 => RelayTransactionHashesReader::new_unchecked(inner).into(),
            4 => GetRelayTransactionsReader::new_unchecked(inner).into(),
            5 => GetBlockTransactionsReader::new_unchecked(inner).into(),
            6 => BlockTransactionsReader::new_unchecked(inner).into(),
            7 => GetBlockProposalReader::new_unchecked(inner).into(),
            8 => BlockProposalReader::new_unchecked(inner).into(),
            _ => unreachable!(),
        }
    }
}
impl molecule::prelude::Builder for RelayMessageBuilder {
    type Entity = RelayMessage;
    fn expected_length(&self) -> usize {
        4 + self.0.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let item_id = (self.0.item_id() as u32).to_le_bytes();
        writer.write_all(&item_id[..])?;
        writer.write_all(self.0.as_slice())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RelayMessage::new_unchecked(inner.into())
    }
}
impl RelayMessageBuilder {
    pub const NAME: &'static str = "RelayMessageBuilder";
    pub fn set<I>(mut self, v: I) -> Self
    where
        I: ::std::convert::Into<RelayMessageUnion>,
    {
        self.0 = v.into();
        self
    }
}
#[derive(Clone)]
pub struct CompactBlock(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct CompactBlockReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for CompactBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for CompactBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for CompactBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "short_ids", self.short_ids())?;
        write!(
            f,
            ", {}: {}",
            "prefilled_transactions",
            self.prefilled_transactions()
        )?;
        write!(f, ", {}: {}", "uncles", self.uncles())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 5 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for CompactBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "short_ids", self.short_ids())?;
        write!(
            f,
            ", {}: {}",
            "prefilled_transactions",
            self.prefilled_transactions()
        )?;
        write!(f, ", {}: {}", "uncles", self.uncles())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 5 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct CompactBlockBuilder {
    pub(crate) header: Header,
    pub(crate) short_ids: ProposalShortIdVec,
    pub(crate) prefilled_transactions: IndexTransactionVec,
    pub(crate) uncles: UncleBlockVec,
    pub(crate) proposals: ProposalShortIdVec,
}
impl ::std::default::Default for CompactBlock {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            100, 1, 0, 0, 24, 0, 0, 0, 84, 1, 0, 0, 88, 1, 0, 0, 92, 1, 0, 0, 96, 1, 0, 0, 60, 1,
            0, 0, 12, 0, 0, 0, 36, 1, 0, 0, 24, 1, 0, 0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0, 0, 0, 96,
            0, 0, 0, 104, 0, 0, 0, 136, 0, 0, 0, 168, 0, 0, 0, 200, 0, 0, 0, 232, 0, 0, 0, 8, 1, 0,
            0, 12, 1, 0, 0, 20, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
        ];
        CompactBlock::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for CompactBlock {
    type Builder = CompactBlockBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CompactBlock(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CompactBlockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .header(self.header())
            .short_ids(self.short_ids())
            .prefilled_transactions(self.prefilled_transactions())
            .uncles(self.uncles())
            .proposals(self.proposals())
    }
}
impl CompactBlock {
    pub const NAME: &'static str = "CompactBlock";
    pub fn as_reader(&self) -> CompactBlockReader<'_> {
        CompactBlockReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 5;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> Header {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Header::new_unchecked(self.0.slice(start, end))
    }
    pub fn short_ids(&self) -> ProposalShortIdVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        ProposalShortIdVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn prefilled_transactions(&self) -> IndexTransactionVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        IndexTransactionVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn uncles(&self) -> UncleBlockVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        UncleBlockVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn proposals(&self) -> ProposalShortIdVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        if count == 5 {
            ProposalShortIdVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[4 + 1]) as usize;
            ProposalShortIdVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CompactBlockReader<'r> {
    type Entity = CompactBlock;
    fn to_entity(&self) -> Self::Entity {
        CompactBlock::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        CompactBlockReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 5;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=5]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        HeaderReader::verify(&slice[offsets[0]..offsets[1]])?;
        ProposalShortIdVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        IndexTransactionVecReader::verify(&slice[offsets[2]..offsets[3]])?;
        UncleBlockVecReader::verify(&slice[offsets[3]..offsets[4]])?;
        ProposalShortIdVecReader::verify(&slice[offsets[4]..offsets[5]])?;
        Ok(())
    }
}
impl<'r> CompactBlockReader<'r> {
    pub const NAME: &'r str = "CompactBlockReader";
    pub const FIELD_COUNT: usize = 5;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> HeaderReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        HeaderReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn short_ids(&self) -> ProposalShortIdVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn prefilled_transactions(&self) -> IndexTransactionVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        IndexTransactionVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn uncles(&self) -> UncleBlockVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        UncleBlockVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proposals(&self) -> ProposalShortIdVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        if count == 5 {
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[4 + 1]) as usize;
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for CompactBlockBuilder {
    type Entity = CompactBlock;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 5 * 4;
        len_header
            + self.header.as_slice().len()
            + self.short_ids.as_slice().len()
            + self.prefilled_transactions.as_slice().len()
            + self.uncles.as_slice().len()
            + self.proposals.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 5 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.header.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.short_ids.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.prefilled_transactions.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.uncles.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.proposals.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.header.as_slice())?;
        writer.write_all(self.short_ids.as_slice())?;
        writer.write_all(self.prefilled_transactions.as_slice())?;
        writer.write_all(self.uncles.as_slice())?;
        writer.write_all(self.proposals.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        CompactBlock::new_unchecked(inner.into())
    }
}
impl CompactBlockBuilder {
    pub const NAME: &'static str = "CompactBlockBuilder";
    pub fn header(mut self, v: Header) -> Self {
        self.header = v;
        self
    }
    pub fn short_ids(mut self, v: ProposalShortIdVec) -> Self {
        self.short_ids = v;
        self
    }
    pub fn prefilled_transactions(mut self, v: IndexTransactionVec) -> Self {
        self.prefilled_transactions = v;
        self
    }
    pub fn uncles(mut self, v: UncleBlockVec) -> Self {
        self.uncles = v;
        self
    }
    pub fn proposals(mut self, v: ProposalShortIdVec) -> Self {
        self.proposals = v;
        self
    }
}
#[derive(Clone)]
pub struct RelayTransaction(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RelayTransactionReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RelayTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RelayTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RelayTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "cycles", self.cycles())?;
        write!(f, ", {}: {}", "transaction", self.transaction())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for RelayTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "cycles", self.cycles())?;
        write!(f, ", {}: {}", "transaction", self.transaction())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct RelayTransactionBuilder {
    pub(crate) cycles: Uint64,
    pub(crate) transaction: Transaction,
}
impl ::std::default::Default for RelayTransaction {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            88, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 12, 0, 0,
            0, 64, 0, 0, 0, 52, 0, 0, 0, 12, 0, 0, 0, 48, 0, 0, 0, 36, 0, 0, 0, 20, 0, 0, 0, 24, 0,
            0, 0, 28, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0,
            0, 0, 4, 0, 0, 0,
        ];
        RelayTransaction::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for RelayTransaction {
    type Builder = RelayTransactionBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RelayTransaction(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RelayTransactionReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .cycles(self.cycles())
            .transaction(self.transaction())
    }
}
impl RelayTransaction {
    pub const NAME: &'static str = "RelayTransaction";
    pub fn as_reader(&self) -> RelayTransactionReader<'_> {
        RelayTransactionReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn cycles(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn transaction(&self) -> Transaction {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Transaction::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Transaction::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RelayTransactionReader<'r> {
    type Entity = RelayTransaction;
    fn to_entity(&self) -> Self::Entity {
        RelayTransaction::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RelayTransactionReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        TransactionReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> RelayTransactionReader<'r> {
    pub const NAME: &'r str = "RelayTransactionReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn cycles(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn transaction(&self) -> TransactionReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            TransactionReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            TransactionReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for RelayTransactionBuilder {
    type Entity = RelayTransaction;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.cycles.as_slice().len() + self.transaction.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.cycles.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transaction.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.cycles.as_slice())?;
        writer.write_all(self.transaction.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RelayTransaction::new_unchecked(inner.into())
    }
}
impl RelayTransactionBuilder {
    pub const NAME: &'static str = "RelayTransactionBuilder";
    pub fn cycles(mut self, v: Uint64) -> Self {
        self.cycles = v;
        self
    }
    pub fn transaction(mut self, v: Transaction) -> Self {
        self.transaction = v;
        self
    }
}
#[derive(Clone)]
pub struct RelayTransactionVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RelayTransactionVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RelayTransactionVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RelayTransactionVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RelayTransactionVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for RelayTransactionVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct RelayTransactionVecBuilder(pub(crate) Vec<RelayTransaction>);
impl molecule::prelude::Entity for RelayTransactionVec {
    type Builder = RelayTransactionVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RelayTransactionVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RelayTransactionVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for RelayTransactionVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        RelayTransactionVec::new_unchecked(v.into())
    }
}
impl RelayTransactionVec {
    pub const NAME: &'static str = "RelayTransactionVec";
    pub fn as_reader(&self) -> RelayTransactionVecReader<'_> {
        RelayTransactionVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<RelayTransaction> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> RelayTransaction {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            RelayTransaction::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            RelayTransaction::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RelayTransactionVecReader<'r> {
    type Entity = RelayTransactionVec;
    fn to_entity(&self) -> Self::Entity {
        RelayTransactionVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RelayTransactionVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            RelayTransactionReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> RelayTransactionVecReader<'r> {
    pub const NAME: &'r str = "RelayTransactionVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<RelayTransactionReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> RelayTransactionReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            RelayTransactionReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            RelayTransactionReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for RelayTransactionVecBuilder {
    type Entity = RelayTransactionVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RelayTransactionVec::new_unchecked(inner.into())
    }
}
impl RelayTransactionVecBuilder {
    pub const NAME: &'static str = "RelayTransactionVecBuilder";
    pub fn set(mut self, v: Vec<RelayTransaction>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: RelayTransaction) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = RelayTransaction>>(
        mut self,
        iter: T,
    ) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct RelayTransactionVecIterator(RelayTransactionVec, usize, usize);
impl ::std::iter::Iterator for RelayTransactionVecIterator {
    type Item = RelayTransaction;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for RelayTransactionVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for RelayTransactionVec {
    type Item = RelayTransaction;
    type IntoIter = RelayTransactionVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        RelayTransactionVecIterator(self, 0, len)
    }
}
impl<'r> RelayTransactionVecReader<'r> {
    pub fn iter(&self) -> RelayTransactionVecReaderIterator<'_, 'r> {
        RelayTransactionVecReaderIterator(&self, 0, self.len())
    }
}
pub struct RelayTransactionVecReaderIterator<'t, 'r>(
    &'t RelayTransactionVecReader<'r>,
    usize,
    usize,
);
impl<'t: 'r, 'r> ::std::iter::Iterator for RelayTransactionVecReaderIterator<'t, 'r> {
    type Item = RelayTransactionReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for RelayTransactionVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct RelayTransactions(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RelayTransactionsReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RelayTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RelayTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RelayTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "transactions", self.transactions())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for RelayTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "transactions", self.transactions())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct RelayTransactionsBuilder {
    pub(crate) transactions: RelayTransactionVec,
}
impl ::std::default::Default for RelayTransactions {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0];
        RelayTransactions::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for RelayTransactions {
    type Builder = RelayTransactionsBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RelayTransactions(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RelayTransactionsReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().transactions(self.transactions())
    }
}
impl RelayTransactions {
    pub const NAME: &'static str = "RelayTransactions";
    pub fn as_reader(&self) -> RelayTransactionsReader<'_> {
        RelayTransactionsReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn transactions(&self) -> RelayTransactionVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            RelayTransactionVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            RelayTransactionVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RelayTransactionsReader<'r> {
    type Entity = RelayTransactions;
    fn to_entity(&self) -> Self::Entity {
        RelayTransactions::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RelayTransactionsReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        RelayTransactionVecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> RelayTransactionsReader<'r> {
    pub const NAME: &'r str = "RelayTransactionsReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn transactions(&self) -> RelayTransactionVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            RelayTransactionVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            RelayTransactionVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for RelayTransactionsBuilder {
    type Entity = RelayTransactions;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.transactions.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transactions.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.transactions.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RelayTransactions::new_unchecked(inner.into())
    }
}
impl RelayTransactionsBuilder {
    pub const NAME: &'static str = "RelayTransactionsBuilder";
    pub fn transactions(mut self, v: RelayTransactionVec) -> Self {
        self.transactions = v;
        self
    }
}
#[derive(Clone)]
pub struct RelayTransactionHashes(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RelayTransactionHashesReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RelayTransactionHashes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RelayTransactionHashesReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RelayTransactionHashes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "tx_hashes", self.tx_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for RelayTransactionHashesReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "tx_hashes", self.tx_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct RelayTransactionHashesBuilder {
    pub(crate) tx_hashes: Byte32Vec,
}
impl ::std::default::Default for RelayTransactionHashes {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0];
        RelayTransactionHashes::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for RelayTransactionHashes {
    type Builder = RelayTransactionHashesBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RelayTransactionHashes(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RelayTransactionHashesReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().tx_hashes(self.tx_hashes())
    }
}
impl RelayTransactionHashes {
    pub const NAME: &'static str = "RelayTransactionHashes";
    pub fn as_reader(&self) -> RelayTransactionHashesReader<'_> {
        RelayTransactionHashesReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn tx_hashes(&self) -> Byte32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Byte32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Byte32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RelayTransactionHashesReader<'r> {
    type Entity = RelayTransactionHashes;
    fn to_entity(&self) -> Self::Entity {
        RelayTransactionHashes::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RelayTransactionHashesReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32VecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> RelayTransactionHashesReader<'r> {
    pub const NAME: &'r str = "RelayTransactionHashesReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn tx_hashes(&self) -> Byte32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Byte32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for RelayTransactionHashesBuilder {
    type Entity = RelayTransactionHashes;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.tx_hashes.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.tx_hashes.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.tx_hashes.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RelayTransactionHashes::new_unchecked(inner.into())
    }
}
impl RelayTransactionHashesBuilder {
    pub const NAME: &'static str = "RelayTransactionHashesBuilder";
    pub fn tx_hashes(mut self, v: Byte32Vec) -> Self {
        self.tx_hashes = v;
        self
    }
}
#[derive(Clone)]
pub struct GetRelayTransactions(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct GetRelayTransactionsReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for GetRelayTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for GetRelayTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for GetRelayTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "tx_hashes", self.tx_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for GetRelayTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "tx_hashes", self.tx_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct GetRelayTransactionsBuilder {
    pub(crate) tx_hashes: Byte32Vec,
}
impl ::std::default::Default for GetRelayTransactions {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0];
        GetRelayTransactions::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for GetRelayTransactions {
    type Builder = GetRelayTransactionsBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        GetRelayTransactions(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        GetRelayTransactionsReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().tx_hashes(self.tx_hashes())
    }
}
impl GetRelayTransactions {
    pub const NAME: &'static str = "GetRelayTransactions";
    pub fn as_reader(&self) -> GetRelayTransactionsReader<'_> {
        GetRelayTransactionsReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn tx_hashes(&self) -> Byte32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Byte32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Byte32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for GetRelayTransactionsReader<'r> {
    type Entity = GetRelayTransactions;
    fn to_entity(&self) -> Self::Entity {
        GetRelayTransactions::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        GetRelayTransactionsReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32VecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> GetRelayTransactionsReader<'r> {
    pub const NAME: &'r str = "GetRelayTransactionsReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn tx_hashes(&self) -> Byte32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Byte32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for GetRelayTransactionsBuilder {
    type Entity = GetRelayTransactions;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.tx_hashes.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.tx_hashes.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.tx_hashes.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        GetRelayTransactions::new_unchecked(inner.into())
    }
}
impl GetRelayTransactionsBuilder {
    pub const NAME: &'static str = "GetRelayTransactionsBuilder";
    pub fn tx_hashes(mut self, v: Byte32Vec) -> Self {
        self.tx_hashes = v;
        self
    }
}
#[derive(Clone)]
pub struct GetBlockTransactions(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct GetBlockTransactionsReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for GetBlockTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for GetBlockTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for GetBlockTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "indexes", self.indexes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for GetBlockTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "indexes", self.indexes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct GetBlockTransactionsBuilder {
    pub(crate) block_hash: Byte32,
    pub(crate) indexes: Uint32Vec,
}
impl ::std::default::Default for GetBlockTransactions {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            48, 0, 0, 0, 12, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        GetBlockTransactions::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for GetBlockTransactions {
    type Builder = GetBlockTransactionsBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        GetBlockTransactions(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        GetBlockTransactionsReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .block_hash(self.block_hash())
            .indexes(self.indexes())
    }
}
impl GetBlockTransactions {
    pub const NAME: &'static str = "GetBlockTransactions";
    pub fn as_reader(&self) -> GetBlockTransactionsReader<'_> {
        GetBlockTransactionsReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn indexes(&self) -> Uint32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Uint32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Uint32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for GetBlockTransactionsReader<'r> {
    type Entity = GetBlockTransactions;
    fn to_entity(&self) -> Self::Entity {
        GetBlockTransactions::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        GetBlockTransactionsReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Uint32VecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> GetBlockTransactionsReader<'r> {
    pub const NAME: &'r str = "GetBlockTransactionsReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn indexes(&self) -> Uint32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Uint32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Uint32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for GetBlockTransactionsBuilder {
    type Entity = GetBlockTransactions;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.block_hash.as_slice().len() + self.indexes.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.indexes.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_hash.as_slice())?;
        writer.write_all(self.indexes.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        GetBlockTransactions::new_unchecked(inner.into())
    }
}
impl GetBlockTransactionsBuilder {
    pub const NAME: &'static str = "GetBlockTransactionsBuilder";
    pub fn block_hash(mut self, v: Byte32) -> Self {
        self.block_hash = v;
        self
    }
    pub fn indexes(mut self, v: Uint32Vec) -> Self {
        self.indexes = v;
        self
    }
}
#[derive(Clone)]
pub struct BlockTransactions(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BlockTransactionsReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for BlockTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BlockTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for BlockTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "transactions", self.transactions())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for BlockTransactionsReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "transactions", self.transactions())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct BlockTransactionsBuilder {
    pub(crate) block_hash: Byte32,
    pub(crate) transactions: TransactionVec,
}
impl ::std::default::Default for BlockTransactions {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            48, 0, 0, 0, 12, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
        ];
        BlockTransactions::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for BlockTransactions {
    type Builder = BlockTransactionsBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        BlockTransactions(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BlockTransactionsReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .block_hash(self.block_hash())
            .transactions(self.transactions())
    }
}
impl BlockTransactions {
    pub const NAME: &'static str = "BlockTransactions";
    pub fn as_reader(&self) -> BlockTransactionsReader<'_> {
        BlockTransactionsReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn transactions(&self) -> TransactionVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            TransactionVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            TransactionVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BlockTransactionsReader<'r> {
    type Entity = BlockTransactions;
    fn to_entity(&self) -> Self::Entity {
        BlockTransactions::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BlockTransactionsReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        TransactionVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> BlockTransactionsReader<'r> {
    pub const NAME: &'r str = "BlockTransactionsReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn transactions(&self) -> TransactionVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            TransactionVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            TransactionVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for BlockTransactionsBuilder {
    type Entity = BlockTransactions;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.block_hash.as_slice().len() + self.transactions.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transactions.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_hash.as_slice())?;
        writer.write_all(self.transactions.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        BlockTransactions::new_unchecked(inner.into())
    }
}
impl BlockTransactionsBuilder {
    pub const NAME: &'static str = "BlockTransactionsBuilder";
    pub fn block_hash(mut self, v: Byte32) -> Self {
        self.block_hash = v;
        self
    }
    pub fn transactions(mut self, v: TransactionVec) -> Self {
        self.transactions = v;
        self
    }
}
#[derive(Clone)]
pub struct GetBlockProposal(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct GetBlockProposalReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for GetBlockProposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for GetBlockProposalReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for GetBlockProposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for GetBlockProposalReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hash", self.block_hash())?;
        write!(f, ", {}: {}", "proposals", self.proposals())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct GetBlockProposalBuilder {
    pub(crate) block_hash: Byte32,
    pub(crate) proposals: ProposalShortIdVec,
}
impl ::std::default::Default for GetBlockProposal {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            48, 0, 0, 0, 12, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        GetBlockProposal::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for GetBlockProposal {
    type Builder = GetBlockProposalBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        GetBlockProposal(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        GetBlockProposalReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .block_hash(self.block_hash())
            .proposals(self.proposals())
    }
}
impl GetBlockProposal {
    pub const NAME: &'static str = "GetBlockProposal";
    pub fn as_reader(&self) -> GetBlockProposalReader<'_> {
        GetBlockProposalReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hash(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn proposals(&self) -> ProposalShortIdVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            ProposalShortIdVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            ProposalShortIdVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for GetBlockProposalReader<'r> {
    type Entity = GetBlockProposal;
    fn to_entity(&self) -> Self::Entity {
        GetBlockProposal::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        GetBlockProposalReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        ProposalShortIdVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> GetBlockProposalReader<'r> {
    pub const NAME: &'r str = "GetBlockProposalReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hash(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proposals(&self) -> ProposalShortIdVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            ProposalShortIdVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for GetBlockProposalBuilder {
    type Entity = GetBlockProposal;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.block_hash.as_slice().len() + self.proposals.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_hash.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.proposals.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_hash.as_slice())?;
        writer.write_all(self.proposals.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        GetBlockProposal::new_unchecked(inner.into())
    }
}
impl GetBlockProposalBuilder {
    pub const NAME: &'static str = "GetBlockProposalBuilder";
    pub fn block_hash(mut self, v: Byte32) -> Self {
        self.block_hash = v;
        self
    }
    pub fn proposals(mut self, v: ProposalShortIdVec) -> Self {
        self.proposals = v;
        self
    }
}
#[derive(Clone)]
pub struct BlockProposal(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct BlockProposalReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for BlockProposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for BlockProposalReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for BlockProposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "transactions", self.transactions())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for BlockProposalReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "transactions", self.transactions())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct BlockProposalBuilder {
    pub(crate) transactions: TransactionVec,
}
impl ::std::default::Default for BlockProposal {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0];
        BlockProposal::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for BlockProposal {
    type Builder = BlockProposalBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        BlockProposal(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BlockProposalReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().transactions(self.transactions())
    }
}
impl BlockProposal {
    pub const NAME: &'static str = "BlockProposal";
    pub fn as_reader(&self) -> BlockProposalReader<'_> {
        BlockProposalReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn transactions(&self) -> TransactionVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            TransactionVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            TransactionVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for BlockProposalReader<'r> {
    type Entity = BlockProposal;
    fn to_entity(&self) -> Self::Entity {
        BlockProposal::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BlockProposalReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        TransactionVecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> BlockProposalReader<'r> {
    pub const NAME: &'r str = "BlockProposalReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn transactions(&self) -> TransactionVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            TransactionVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            TransactionVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for BlockProposalBuilder {
    type Entity = BlockProposal;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.transactions.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transactions.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.transactions.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        BlockProposal::new_unchecked(inner.into())
    }
}
impl BlockProposalBuilder {
    pub const NAME: &'static str = "BlockProposalBuilder";
    pub fn transactions(mut self, v: TransactionVec) -> Self {
        self.transactions = v;
        self
    }
}
#[derive(Clone)]
pub struct IndexTransaction(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct IndexTransactionReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for IndexTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for IndexTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for IndexTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "index", self.index())?;
        write!(f, ", {}: {}", "transaction", self.transaction())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for IndexTransactionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "index", self.index())?;
        write!(f, ", {}: {}", "transaction", self.transaction())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct IndexTransactionBuilder {
    pub(crate) index: Uint32,
    pub(crate) transaction: Transaction,
}
impl ::std::default::Default for IndexTransaction {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            84, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 12, 0, 0, 0, 64, 0, 0,
            0, 52, 0, 0, 0, 12, 0, 0, 0, 48, 0, 0, 0, 36, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 28, 0,
            0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0,
            0,
        ];
        IndexTransaction::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for IndexTransaction {
    type Builder = IndexTransactionBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        IndexTransaction(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        IndexTransactionReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .index(self.index())
            .transaction(self.transaction())
    }
}
impl IndexTransaction {
    pub const NAME: &'static str = "IndexTransaction";
    pub fn as_reader(&self) -> IndexTransactionReader<'_> {
        IndexTransactionReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn index(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn transaction(&self) -> Transaction {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Transaction::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Transaction::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for IndexTransactionReader<'r> {
    type Entity = IndexTransaction;
    fn to_entity(&self) -> Self::Entity {
        IndexTransaction::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        IndexTransactionReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        TransactionReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> IndexTransactionReader<'r> {
    pub const NAME: &'r str = "IndexTransactionReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn index(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn transaction(&self) -> TransactionReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            TransactionReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            TransactionReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for IndexTransactionBuilder {
    type Entity = IndexTransaction;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.index.as_slice().len() + self.transaction.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.index.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transaction.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.index.as_slice())?;
        writer.write_all(self.transaction.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        IndexTransaction::new_unchecked(inner.into())
    }
}
impl IndexTransactionBuilder {
    pub const NAME: &'static str = "IndexTransactionBuilder";
    pub fn index(mut self, v: Uint32) -> Self {
        self.index = v;
        self
    }
    pub fn transaction(mut self, v: Transaction) -> Self {
        self.transaction = v;
        self
    }
}
#[derive(Clone)]
pub struct IndexTransactionVec(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct IndexTransactionVecReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for IndexTransactionVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for IndexTransactionVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for IndexTransactionVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ::std::fmt::Display for IndexTransactionVecReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
#[derive(Debug, Default)]
pub struct IndexTransactionVecBuilder(pub(crate) Vec<IndexTransaction>);
impl molecule::prelude::Entity for IndexTransactionVec {
    type Builder = IndexTransactionVecBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        IndexTransactionVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        IndexTransactionVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
impl ::std::default::Default for IndexTransactionVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        IndexTransactionVec::new_unchecked(v.into())
    }
}
impl IndexTransactionVec {
    pub const NAME: &'static str = "IndexTransactionVec";
    pub fn as_reader(&self) -> IndexTransactionVecReader<'_> {
        IndexTransactionVecReader::new_unchecked(self.as_slice())
    }
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<IndexTransaction> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> IndexTransaction {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            IndexTransaction::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            IndexTransaction::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for IndexTransactionVecReader<'r> {
    type Entity = IndexTransactionVec;
    fn to_entity(&self) -> Self::Entity {
        IndexTransactionVec::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        IndexTransactionVecReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        if total_size == 4 {
            return Ok(());
        }
        if total_size < 4 + 4 {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), 8, total_size);
            Err(err)?;
        }
        let offset_first = u32::from_le(ptr[1]) as usize;
        if offset_first % 4 != 0 {
            let err = VerificationError::FirstOffsetIsBroken(Self::NAME.to_owned(), offset_first);
            Err(err)?;
        }
        if offset_first < 4 + 4 {
            let err = VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), 8, offset_first);
            Err(err)?;
        }
        let item_count = offset_first / 4 - 1;
        let expected = 4 + 4 * item_count;
        if total_size < expected {
            let err = VerificationError::DataIsShort(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..(item_count + 1)]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        for i in 0..=(offsets.len() - 2) {
            let start = offsets[i];
            let end = offsets[i + 1];
            IndexTransactionReader::verify(&slice[start..end])?;
        }
        Ok(())
    }
}
impl<'r> IndexTransactionVecReader<'r> {
    pub const NAME: &'r str = "IndexTransactionVecReader";
    pub fn offsets(&self) -> (usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        (bytes_len, &ptr[1..])
    }
    pub fn len(&self) -> usize {
        let (bytes_len, offsets) = self.offsets();
        if bytes_len == 4 {
            0
        } else {
            let first = u32::from_le(offsets[0]) as usize;
            (first - 4) / 4
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<IndexTransactionReader<'_>> {
        let len = self.len();
        if idx >= len {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> IndexTransactionReader<'_> {
        let len = self.len();
        let (_, offsets) = self.offsets();
        let start = u32::from_le(offsets[idx]) as usize;
        if idx == len - 1 {
            IndexTransactionReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[idx + 1]) as usize;
            IndexTransactionReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for IndexTransactionVecBuilder {
    type Entity = IndexTransactionVec;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 4 * self.0.len();
        len_header
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 4 * self.0.len();
        for inner in &self.0[..] {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += inner.as_slice().len();
        }
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        IndexTransactionVec::new_unchecked(inner.into())
    }
}
impl IndexTransactionVecBuilder {
    pub const NAME: &'static str = "IndexTransactionVecBuilder";
    pub fn set(mut self, v: Vec<IndexTransaction>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: IndexTransaction) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::std::iter::IntoIterator<Item = IndexTransaction>>(
        mut self,
        iter: T,
    ) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
pub struct IndexTransactionVecIterator(IndexTransactionVec, usize, usize);
impl ::std::iter::Iterator for IndexTransactionVecIterator {
    type Item = IndexTransaction;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::std::iter::ExactSizeIterator for IndexTransactionVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::std::iter::IntoIterator for IndexTransactionVec {
    type Item = IndexTransaction;
    type IntoIter = IndexTransactionVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        IndexTransactionVecIterator(self, 0, len)
    }
}
impl<'r> IndexTransactionVecReader<'r> {
    pub fn iter(&self) -> IndexTransactionVecReaderIterator<'_, 'r> {
        IndexTransactionVecReaderIterator(&self, 0, self.len())
    }
}
pub struct IndexTransactionVecReaderIterator<'t, 'r>(
    &'t IndexTransactionVecReader<'r>,
    usize,
    usize,
);
impl<'t: 'r, 'r> ::std::iter::Iterator for IndexTransactionVecReaderIterator<'t, 'r> {
    type Item = IndexTransactionReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for IndexTransactionVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct SyncMessage(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct SyncMessageReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for SyncMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for SyncMessageReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for SyncMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
impl<'r> ::std::fmt::Display for SyncMessageReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
#[derive(Debug, Clone)]
pub enum SyncMessageUnion {
    GetHeaders(GetHeaders),
    SendHeaders(SendHeaders),
    GetBlocks(GetBlocks),
    SendBlock(SendBlock),
    SetFilter(SetFilter),
    AddFilter(AddFilter),
    ClearFilter(ClearFilter),
    FilteredBlock(FilteredBlock),
    InIBD(InIBD),
}
#[derive(Debug, Clone, Copy)]
pub enum SyncMessageUnionReader<'r> {
    GetHeaders(GetHeadersReader<'r>),
    SendHeaders(SendHeadersReader<'r>),
    GetBlocks(GetBlocksReader<'r>),
    SendBlock(SendBlockReader<'r>),
    SetFilter(SetFilterReader<'r>),
    AddFilter(AddFilterReader<'r>),
    ClearFilter(ClearFilterReader<'r>),
    FilteredBlock(FilteredBlockReader<'r>),
    InIBD(InIBDReader<'r>),
}
impl ::std::default::Default for SyncMessageUnion {
    fn default() -> Self {
        SyncMessageUnion::GetHeaders(::std::default::Default::default())
    }
}
impl ::std::fmt::Display for SyncMessageUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            SyncMessageUnion::GetHeaders(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, GetHeaders::NAME, item)
            }
            SyncMessageUnion::SendHeaders(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SendHeaders::NAME, item)
            }
            SyncMessageUnion::GetBlocks(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, GetBlocks::NAME, item)
            }
            SyncMessageUnion::SendBlock(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SendBlock::NAME, item)
            }
            SyncMessageUnion::SetFilter(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SetFilter::NAME, item)
            }
            SyncMessageUnion::AddFilter(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, AddFilter::NAME, item)
            }
            SyncMessageUnion::ClearFilter(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, ClearFilter::NAME, item)
            }
            SyncMessageUnion::FilteredBlock(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, FilteredBlock::NAME, item)
            }
            SyncMessageUnion::InIBD(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, InIBD::NAME, item)
            }
        }
    }
}
impl<'r> ::std::fmt::Display for SyncMessageUnionReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            SyncMessageUnionReader::GetHeaders(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, GetHeaders::NAME, item)
            }
            SyncMessageUnionReader::SendHeaders(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SendHeaders::NAME, item)
            }
            SyncMessageUnionReader::GetBlocks(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, GetBlocks::NAME, item)
            }
            SyncMessageUnionReader::SendBlock(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SendBlock::NAME, item)
            }
            SyncMessageUnionReader::SetFilter(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SetFilter::NAME, item)
            }
            SyncMessageUnionReader::AddFilter(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, AddFilter::NAME, item)
            }
            SyncMessageUnionReader::ClearFilter(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, ClearFilter::NAME, item)
            }
            SyncMessageUnionReader::FilteredBlock(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, FilteredBlock::NAME, item)
            }
            SyncMessageUnionReader::InIBD(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, InIBD::NAME, item)
            }
        }
    }
}
impl SyncMessageUnion {
    pub(crate) fn display_inner(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            SyncMessageUnion::GetHeaders(ref item) => write!(f, "{}", item),
            SyncMessageUnion::SendHeaders(ref item) => write!(f, "{}", item),
            SyncMessageUnion::GetBlocks(ref item) => write!(f, "{}", item),
            SyncMessageUnion::SendBlock(ref item) => write!(f, "{}", item),
            SyncMessageUnion::SetFilter(ref item) => write!(f, "{}", item),
            SyncMessageUnion::AddFilter(ref item) => write!(f, "{}", item),
            SyncMessageUnion::ClearFilter(ref item) => write!(f, "{}", item),
            SyncMessageUnion::FilteredBlock(ref item) => write!(f, "{}", item),
            SyncMessageUnion::InIBD(ref item) => write!(f, "{}", item),
        }
    }
}
impl<'r> SyncMessageUnionReader<'r> {
    pub(crate) fn display_inner(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            SyncMessageUnionReader::GetHeaders(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::SendHeaders(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::GetBlocks(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::SendBlock(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::SetFilter(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::AddFilter(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::ClearFilter(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::FilteredBlock(ref item) => write!(f, "{}", item),
            SyncMessageUnionReader::InIBD(ref item) => write!(f, "{}", item),
        }
    }
}
impl ::std::convert::From<GetHeaders> for SyncMessageUnion {
    fn from(item: GetHeaders) -> Self {
        SyncMessageUnion::GetHeaders(item)
    }
}
impl ::std::convert::From<SendHeaders> for SyncMessageUnion {
    fn from(item: SendHeaders) -> Self {
        SyncMessageUnion::SendHeaders(item)
    }
}
impl ::std::convert::From<GetBlocks> for SyncMessageUnion {
    fn from(item: GetBlocks) -> Self {
        SyncMessageUnion::GetBlocks(item)
    }
}
impl ::std::convert::From<SendBlock> for SyncMessageUnion {
    fn from(item: SendBlock) -> Self {
        SyncMessageUnion::SendBlock(item)
    }
}
impl ::std::convert::From<SetFilter> for SyncMessageUnion {
    fn from(item: SetFilter) -> Self {
        SyncMessageUnion::SetFilter(item)
    }
}
impl ::std::convert::From<AddFilter> for SyncMessageUnion {
    fn from(item: AddFilter) -> Self {
        SyncMessageUnion::AddFilter(item)
    }
}
impl ::std::convert::From<ClearFilter> for SyncMessageUnion {
    fn from(item: ClearFilter) -> Self {
        SyncMessageUnion::ClearFilter(item)
    }
}
impl ::std::convert::From<FilteredBlock> for SyncMessageUnion {
    fn from(item: FilteredBlock) -> Self {
        SyncMessageUnion::FilteredBlock(item)
    }
}
impl ::std::convert::From<InIBD> for SyncMessageUnion {
    fn from(item: InIBD) -> Self {
        SyncMessageUnion::InIBD(item)
    }
}
impl<'r> ::std::convert::From<GetHeadersReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: GetHeadersReader<'r>) -> Self {
        SyncMessageUnionReader::GetHeaders(item)
    }
}
impl<'r> ::std::convert::From<SendHeadersReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: SendHeadersReader<'r>) -> Self {
        SyncMessageUnionReader::SendHeaders(item)
    }
}
impl<'r> ::std::convert::From<GetBlocksReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: GetBlocksReader<'r>) -> Self {
        SyncMessageUnionReader::GetBlocks(item)
    }
}
impl<'r> ::std::convert::From<SendBlockReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: SendBlockReader<'r>) -> Self {
        SyncMessageUnionReader::SendBlock(item)
    }
}
impl<'r> ::std::convert::From<SetFilterReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: SetFilterReader<'r>) -> Self {
        SyncMessageUnionReader::SetFilter(item)
    }
}
impl<'r> ::std::convert::From<AddFilterReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: AddFilterReader<'r>) -> Self {
        SyncMessageUnionReader::AddFilter(item)
    }
}
impl<'r> ::std::convert::From<ClearFilterReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: ClearFilterReader<'r>) -> Self {
        SyncMessageUnionReader::ClearFilter(item)
    }
}
impl<'r> ::std::convert::From<FilteredBlockReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: FilteredBlockReader<'r>) -> Self {
        SyncMessageUnionReader::FilteredBlock(item)
    }
}
impl<'r> ::std::convert::From<InIBDReader<'r>> for SyncMessageUnionReader<'r> {
    fn from(item: InIBDReader<'r>) -> Self {
        SyncMessageUnionReader::InIBD(item)
    }
}
impl SyncMessageUnion {
    pub const NAME: &'static str = "SyncMessageUnion";
    pub fn as_bytes(&self) -> molecule::bytes::Bytes {
        match self {
            SyncMessageUnion::GetHeaders(item) => item.as_bytes(),
            SyncMessageUnion::SendHeaders(item) => item.as_bytes(),
            SyncMessageUnion::GetBlocks(item) => item.as_bytes(),
            SyncMessageUnion::SendBlock(item) => item.as_bytes(),
            SyncMessageUnion::SetFilter(item) => item.as_bytes(),
            SyncMessageUnion::AddFilter(item) => item.as_bytes(),
            SyncMessageUnion::ClearFilter(item) => item.as_bytes(),
            SyncMessageUnion::FilteredBlock(item) => item.as_bytes(),
            SyncMessageUnion::InIBD(item) => item.as_bytes(),
        }
    }
    pub fn as_slice(&self) -> &[u8] {
        match self {
            SyncMessageUnion::GetHeaders(item) => item.as_slice(),
            SyncMessageUnion::SendHeaders(item) => item.as_slice(),
            SyncMessageUnion::GetBlocks(item) => item.as_slice(),
            SyncMessageUnion::SendBlock(item) => item.as_slice(),
            SyncMessageUnion::SetFilter(item) => item.as_slice(),
            SyncMessageUnion::AddFilter(item) => item.as_slice(),
            SyncMessageUnion::ClearFilter(item) => item.as_slice(),
            SyncMessageUnion::FilteredBlock(item) => item.as_slice(),
            SyncMessageUnion::InIBD(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> usize {
        match self {
            SyncMessageUnion::GetHeaders(_) => 1,
            SyncMessageUnion::SendHeaders(_) => 2,
            SyncMessageUnion::GetBlocks(_) => 3,
            SyncMessageUnion::SendBlock(_) => 4,
            SyncMessageUnion::SetFilter(_) => 5,
            SyncMessageUnion::AddFilter(_) => 6,
            SyncMessageUnion::ClearFilter(_) => 7,
            SyncMessageUnion::FilteredBlock(_) => 8,
            SyncMessageUnion::InIBD(_) => 9,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            SyncMessageUnion::GetHeaders(_) => "GetHeaders",
            SyncMessageUnion::SendHeaders(_) => "SendHeaders",
            SyncMessageUnion::GetBlocks(_) => "GetBlocks",
            SyncMessageUnion::SendBlock(_) => "SendBlock",
            SyncMessageUnion::SetFilter(_) => "SetFilter",
            SyncMessageUnion::AddFilter(_) => "AddFilter",
            SyncMessageUnion::ClearFilter(_) => "ClearFilter",
            SyncMessageUnion::FilteredBlock(_) => "FilteredBlock",
            SyncMessageUnion::InIBD(_) => "InIBD",
        }
    }
    pub fn as_reader(&self) -> SyncMessageUnionReader<'_> {
        match self {
            SyncMessageUnion::GetHeaders(item) => item.as_reader().into(),
            SyncMessageUnion::SendHeaders(item) => item.as_reader().into(),
            SyncMessageUnion::GetBlocks(item) => item.as_reader().into(),
            SyncMessageUnion::SendBlock(item) => item.as_reader().into(),
            SyncMessageUnion::SetFilter(item) => item.as_reader().into(),
            SyncMessageUnion::AddFilter(item) => item.as_reader().into(),
            SyncMessageUnion::ClearFilter(item) => item.as_reader().into(),
            SyncMessageUnion::FilteredBlock(item) => item.as_reader().into(),
            SyncMessageUnion::InIBD(item) => item.as_reader().into(),
        }
    }
}
impl<'r> SyncMessageUnionReader<'r> {
    pub const NAME: &'r str = "SyncMessageUnionReader";
    pub fn as_slice(&self) -> &[u8] {
        match self {
            SyncMessageUnionReader::GetHeaders(item) => item.as_slice(),
            SyncMessageUnionReader::SendHeaders(item) => item.as_slice(),
            SyncMessageUnionReader::GetBlocks(item) => item.as_slice(),
            SyncMessageUnionReader::SendBlock(item) => item.as_slice(),
            SyncMessageUnionReader::SetFilter(item) => item.as_slice(),
            SyncMessageUnionReader::AddFilter(item) => item.as_slice(),
            SyncMessageUnionReader::ClearFilter(item) => item.as_slice(),
            SyncMessageUnionReader::FilteredBlock(item) => item.as_slice(),
            SyncMessageUnionReader::InIBD(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> usize {
        match self {
            SyncMessageUnionReader::GetHeaders(_) => 1,
            SyncMessageUnionReader::SendHeaders(_) => 2,
            SyncMessageUnionReader::GetBlocks(_) => 3,
            SyncMessageUnionReader::SendBlock(_) => 4,
            SyncMessageUnionReader::SetFilter(_) => 5,
            SyncMessageUnionReader::AddFilter(_) => 6,
            SyncMessageUnionReader::ClearFilter(_) => 7,
            SyncMessageUnionReader::FilteredBlock(_) => 8,
            SyncMessageUnionReader::InIBD(_) => 9,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            SyncMessageUnionReader::GetHeaders(_) => "GetHeaders",
            SyncMessageUnionReader::SendHeaders(_) => "SendHeaders",
            SyncMessageUnionReader::GetBlocks(_) => "GetBlocks",
            SyncMessageUnionReader::SendBlock(_) => "SendBlock",
            SyncMessageUnionReader::SetFilter(_) => "SetFilter",
            SyncMessageUnionReader::AddFilter(_) => "AddFilter",
            SyncMessageUnionReader::ClearFilter(_) => "ClearFilter",
            SyncMessageUnionReader::FilteredBlock(_) => "FilteredBlock",
            SyncMessageUnionReader::InIBD(_) => "InIBD",
        }
    }
}
#[derive(Debug, Default)]
pub struct SyncMessageBuilder(pub(crate) SyncMessageUnion);
impl molecule::prelude::Entity for SyncMessage {
    type Builder = SyncMessageBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SyncMessage(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SyncMessageReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_enum())
    }
}
impl ::std::default::Default for SyncMessage {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        SyncMessage::new_unchecked(v.into())
    }
}
impl SyncMessage {
    pub const NAME: &'static str = "SyncMessage";
    pub fn as_reader(&self) -> SyncMessageReader<'_> {
        SyncMessageReader::new_unchecked(self.as_slice())
    }
    pub const ITEM_COUNT: usize = 9;
    pub fn item_id(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn to_enum(&self) -> SyncMessageUnion {
        let inner = self.0.slice_from(4);
        match self.item_id() {
            1 => GetHeaders::new_unchecked(inner).into(),
            2 => SendHeaders::new_unchecked(inner).into(),
            3 => GetBlocks::new_unchecked(inner).into(),
            4 => SendBlock::new_unchecked(inner).into(),
            5 => SetFilter::new_unchecked(inner).into(),
            6 => AddFilter::new_unchecked(inner).into(),
            7 => ClearFilter::new_unchecked(inner).into(),
            8 => FilteredBlock::new_unchecked(inner).into(),
            9 => InIBD::new_unchecked(inner).into(),
            _ => unreachable!(),
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for SyncMessageReader<'r> {
    type Entity = SyncMessage;
    fn to_entity(&self) -> Self::Entity {
        SyncMessage::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        SyncMessageReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        if slice.len() < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, slice.len());
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let item_id = u32::from_le(ptr[0]) as usize;
        match item_id {
            1 => GetHeadersReader::verify(&slice[4..]),
            2 => SendHeadersReader::verify(&slice[4..]),
            3 => GetBlocksReader::verify(&slice[4..]),
            4 => SendBlockReader::verify(&slice[4..]),
            5 => SetFilterReader::verify(&slice[4..]),
            6 => AddFilterReader::verify(&slice[4..]),
            7 => ClearFilterReader::verify(&slice[4..]),
            8 => FilteredBlockReader::verify(&slice[4..]),
            9 => InIBDReader::verify(&slice[4..]),
            _ => {
                let err = VerificationError::UnknownItem(Self::NAME.to_owned(), 9, item_id);
                Err(err)
            }
        }?;
        Ok(())
    }
}
impl<'r> SyncMessageReader<'r> {
    pub const NAME: &'r str = "SyncMessageReader";
    pub const ITEM_COUNT: usize = 9;
    pub fn item_id(&self) -> usize {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le }) as usize
    }
    pub fn to_enum(&self) -> SyncMessageUnionReader<'_> {
        let inner = &self.as_slice()[4..];
        match self.item_id() {
            1 => GetHeadersReader::new_unchecked(inner).into(),
            2 => SendHeadersReader::new_unchecked(inner).into(),
            3 => GetBlocksReader::new_unchecked(inner).into(),
            4 => SendBlockReader::new_unchecked(inner).into(),
            5 => SetFilterReader::new_unchecked(inner).into(),
            6 => AddFilterReader::new_unchecked(inner).into(),
            7 => ClearFilterReader::new_unchecked(inner).into(),
            8 => FilteredBlockReader::new_unchecked(inner).into(),
            9 => InIBDReader::new_unchecked(inner).into(),
            _ => unreachable!(),
        }
    }
}
impl molecule::prelude::Builder for SyncMessageBuilder {
    type Entity = SyncMessage;
    fn expected_length(&self) -> usize {
        4 + self.0.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let item_id = (self.0.item_id() as u32).to_le_bytes();
        writer.write_all(&item_id[..])?;
        writer.write_all(self.0.as_slice())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        SyncMessage::new_unchecked(inner.into())
    }
}
impl SyncMessageBuilder {
    pub const NAME: &'static str = "SyncMessageBuilder";
    pub fn set<I>(mut self, v: I) -> Self
    where
        I: ::std::convert::Into<SyncMessageUnion>,
    {
        self.0 = v.into();
        self
    }
}
#[derive(Clone)]
pub struct GetHeaders(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct GetHeadersReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for GetHeaders {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for GetHeadersReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for GetHeaders {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "hash_stop", self.hash_stop())?;
        write!(
            f,
            ", {}: {}",
            "block_locator_hashes",
            self.block_locator_hashes()
        )?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for GetHeadersReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "hash_stop", self.hash_stop())?;
        write!(
            f,
            ", {}: {}",
            "block_locator_hashes",
            self.block_locator_hashes()
        )?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct GetHeadersBuilder {
    pub(crate) hash_stop: Byte32,
    pub(crate) block_locator_hashes: Byte32Vec,
}
impl ::std::default::Default for GetHeaders {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            48, 0, 0, 0, 12, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        GetHeaders::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for GetHeaders {
    type Builder = GetHeadersBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        GetHeaders(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        GetHeadersReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .hash_stop(self.hash_stop())
            .block_locator_hashes(self.block_locator_hashes())
    }
}
impl GetHeaders {
    pub const NAME: &'static str = "GetHeaders";
    pub fn as_reader(&self) -> GetHeadersReader<'_> {
        GetHeadersReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn hash_stop(&self) -> Byte32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32::new_unchecked(self.0.slice(start, end))
    }
    pub fn block_locator_hashes(&self) -> Byte32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for GetHeadersReader<'r> {
    type Entity = GetHeaders;
    fn to_entity(&self) -> Self::Entity {
        GetHeaders::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        GetHeadersReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32VecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> GetHeadersReader<'r> {
    pub const NAME: &'r str = "GetHeadersReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn hash_stop(&self) -> Byte32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn block_locator_hashes(&self) -> Byte32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for GetHeadersBuilder {
    type Entity = GetHeaders;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.hash_stop.as_slice().len() + self.block_locator_hashes.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.hash_stop.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_locator_hashes.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.hash_stop.as_slice())?;
        writer.write_all(self.block_locator_hashes.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        GetHeaders::new_unchecked(inner.into())
    }
}
impl GetHeadersBuilder {
    pub const NAME: &'static str = "GetHeadersBuilder";
    pub fn hash_stop(mut self, v: Byte32) -> Self {
        self.hash_stop = v;
        self
    }
    pub fn block_locator_hashes(mut self, v: Byte32Vec) -> Self {
        self.block_locator_hashes = v;
        self
    }
}
#[derive(Clone)]
pub struct GetBlocks(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct GetBlocksReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for GetBlocks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for GetBlocksReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for GetBlocks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hashes", self.block_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for GetBlocksReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block_hashes", self.block_hashes())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct GetBlocksBuilder {
    pub(crate) block_hashes: Byte32Vec,
}
impl ::std::default::Default for GetBlocks {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0];
        GetBlocks::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for GetBlocks {
    type Builder = GetBlocksBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        GetBlocks(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        GetBlocksReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().block_hashes(self.block_hashes())
    }
}
impl GetBlocks {
    pub const NAME: &'static str = "GetBlocks";
    pub fn as_reader(&self) -> GetBlocksReader<'_> {
        GetBlocksReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hashes(&self) -> Byte32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Byte32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Byte32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for GetBlocksReader<'r> {
    type Entity = GetBlocks;
    fn to_entity(&self) -> Self::Entity {
        GetBlocks::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        GetBlocksReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Byte32VecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> GetBlocksReader<'r> {
    pub const NAME: &'r str = "GetBlocksReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block_hashes(&self) -> Byte32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Byte32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for GetBlocksBuilder {
    type Entity = GetBlocks;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.block_hashes.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block_hashes.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block_hashes.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        GetBlocks::new_unchecked(inner.into())
    }
}
impl GetBlocksBuilder {
    pub const NAME: &'static str = "GetBlocksBuilder";
    pub fn block_hashes(mut self, v: Byte32Vec) -> Self {
        self.block_hashes = v;
        self
    }
}
#[derive(Clone)]
pub struct SendHeaders(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct SendHeadersReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for SendHeaders {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for SendHeadersReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for SendHeaders {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "headers", self.headers())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for SendHeadersReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "headers", self.headers())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct SendHeadersBuilder {
    pub(crate) headers: HeaderVec,
}
impl ::std::default::Default for SendHeaders {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0];
        SendHeaders::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for SendHeaders {
    type Builder = SendHeadersBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SendHeaders(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SendHeadersReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().headers(self.headers())
    }
}
impl SendHeaders {
    pub const NAME: &'static str = "SendHeaders";
    pub fn as_reader(&self) -> SendHeadersReader<'_> {
        SendHeadersReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn headers(&self) -> HeaderVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            HeaderVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            HeaderVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for SendHeadersReader<'r> {
    type Entity = SendHeaders;
    fn to_entity(&self) -> Self::Entity {
        SendHeaders::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        SendHeadersReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        HeaderVecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> SendHeadersReader<'r> {
    pub const NAME: &'r str = "SendHeadersReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn headers(&self) -> HeaderVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            HeaderVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            HeaderVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for SendHeadersBuilder {
    type Entity = SendHeaders;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.headers.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.headers.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.headers.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        SendHeaders::new_unchecked(inner.into())
    }
}
impl SendHeadersBuilder {
    pub const NAME: &'static str = "SendHeadersBuilder";
    pub fn headers(mut self, v: HeaderVec) -> Self {
        self.headers = v;
        self
    }
}
#[derive(Clone)]
pub struct SendBlock(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct SendBlockReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for SendBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for SendBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for SendBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block", self.block())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for SendBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "block", self.block())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct SendBlockBuilder {
    pub(crate) block: Block,
}
impl ::std::default::Default for SendBlock {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            100, 1, 0, 0, 8, 0, 0, 0, 92, 1, 0, 0, 20, 0, 0, 0, 80, 1, 0, 0, 84, 1, 0, 0, 88, 1, 0,
            0, 60, 1, 0, 0, 12, 0, 0, 0, 36, 1, 0, 0, 24, 1, 0, 0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0,
            0, 0, 96, 0, 0, 0, 104, 0, 0, 0, 136, 0, 0, 0, 168, 0, 0, 0, 200, 0, 0, 0, 232, 0, 0,
            0, 8, 1, 0, 0, 12, 1, 0, 0, 20, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
        ];
        SendBlock::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for SendBlock {
    type Builder = SendBlockBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SendBlock(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SendBlockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().block(self.block())
    }
}
impl SendBlock {
    pub const NAME: &'static str = "SendBlock";
    pub fn as_reader(&self) -> SendBlockReader<'_> {
        SendBlockReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block(&self) -> Block {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Block::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Block::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for SendBlockReader<'r> {
    type Entity = SendBlock;
    fn to_entity(&self) -> Self::Entity {
        SendBlock::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        SendBlockReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        BlockReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> SendBlockReader<'r> {
    pub const NAME: &'r str = "SendBlockReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn block(&self) -> BlockReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            BlockReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            BlockReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for SendBlockBuilder {
    type Entity = SendBlock;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.block.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.block.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.block.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        SendBlock::new_unchecked(inner.into())
    }
}
impl SendBlockBuilder {
    pub const NAME: &'static str = "SendBlockBuilder";
    pub fn block(mut self, v: Block) -> Self {
        self.block = v;
        self
    }
}
#[derive(Clone)]
pub struct SetFilter(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct SetFilterReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for SetFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for SetFilterReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for SetFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "hash_seed", self.hash_seed())?;
        write!(f, ", {}: {}", "num_hashes", self.num_hashes())?;
        write!(f, ", {}: {}", "filter", self.filter())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for SetFilterReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "hash_seed", self.hash_seed())?;
        write!(f, ", {}: {}", "num_hashes", self.num_hashes())?;
        write!(f, ", {}: {}", "filter", self.filter())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct SetFilterBuilder {
    pub(crate) hash_seed: Uint32,
    pub(crate) num_hashes: u8,
    pub(crate) filter: Bytes,
}
impl ::std::default::Default for SetFilter {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            25, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        SetFilter::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for SetFilter {
    type Builder = SetFilterBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SetFilter(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SetFilterReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .hash_seed(self.hash_seed())
            .num_hashes(self.num_hashes())
            .filter(self.filter())
    }
}
impl SetFilter {
    pub const NAME: &'static str = "SetFilter";
    pub fn as_reader(&self) -> SetFilterReader<'_> {
        SetFilterReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn hash_seed(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn num_hashes(&self) -> u8 {
        let (_, _, offsets) = Self::field_offsets(self);
        let offset = u32::from_le(offsets[1]) as usize;
        self.0[offset]
    }
    pub fn filter(&self) -> Bytes {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for SetFilterReader<'r> {
    type Entity = SetFilter;
    fn to_entity(&self) -> Self::Entity {
        SetFilter::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        SetFilterReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        if offsets[1] + 1 != offsets[2] {
            let err = VerificationError::FieldIsBroken(Self::NAME.to_owned(), 1);
            Err(err)?;
        }
        BytesReader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> SetFilterReader<'r> {
    pub const NAME: &'r str = "SetFilterReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn hash_seed(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn num_hashes(&self) -> u8 {
        let (_, _, offsets) = Self::field_offsets(self);
        let offset = u32::from_le(offsets[1]) as usize;
        self.0[offset]
    }
    pub fn filter(&self) -> BytesReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for SetFilterBuilder {
    type Entity = SetFilter;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header + self.hash_seed.as_slice().len() + 1 + self.filter.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.hash_seed.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += 1;
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.filter.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.hash_seed.as_slice())?;
        writer.write_all(&[self.num_hashes])?;
        writer.write_all(self.filter.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        SetFilter::new_unchecked(inner.into())
    }
}
impl SetFilterBuilder {
    pub const NAME: &'static str = "SetFilterBuilder";
    pub fn hash_seed(mut self, v: Uint32) -> Self {
        self.hash_seed = v;
        self
    }
    pub fn num_hashes(mut self, v: u8) -> Self {
        self.num_hashes = v;
        self
    }
    pub fn filter(mut self, v: Bytes) -> Self {
        self.filter = v;
        self
    }
}
#[derive(Clone)]
pub struct AddFilter(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct AddFilterReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for AddFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for AddFilterReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for AddFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "filter", self.filter())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for AddFilterReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "filter", self.filter())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct AddFilterBuilder {
    pub(crate) filter: Bytes,
}
impl ::std::default::Default for AddFilter {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0];
        AddFilter::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for AddFilter {
    type Builder = AddFilterBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        AddFilter(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        AddFilterReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().filter(self.filter())
    }
}
impl AddFilter {
    pub const NAME: &'static str = "AddFilter";
    pub fn as_reader(&self) -> AddFilterReader<'_> {
        AddFilterReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn filter(&self) -> Bytes {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for AddFilterReader<'r> {
    type Entity = AddFilter;
    fn to_entity(&self) -> Self::Entity {
        AddFilter::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        AddFilterReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        BytesReader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> AddFilterReader<'r> {
    pub const NAME: &'r str = "AddFilterReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn filter(&self) -> BytesReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for AddFilterBuilder {
    type Entity = AddFilter;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.filter.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.filter.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.filter.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        AddFilter::new_unchecked(inner.into())
    }
}
impl AddFilterBuilder {
    pub const NAME: &'static str = "AddFilterBuilder";
    pub fn filter(mut self, v: Bytes) -> Self {
        self.filter = v;
        self
    }
}
#[derive(Clone)]
pub struct ClearFilter(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct ClearFilterReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for ClearFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for ClearFilterReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for ClearFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 0 {
            write!(f, "..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for ClearFilterReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 0 {
            write!(f, "..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct ClearFilterBuilder {}
impl ::std::default::Default for ClearFilter {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        ClearFilter::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for ClearFilter {
    type Builder = ClearFilterBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ClearFilter(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ClearFilterReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
    }
}
impl ClearFilter {
    pub const NAME: &'static str = "ClearFilter";
    pub fn as_reader(&self) -> ClearFilterReader<'_> {
        ClearFilterReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 0;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
}
impl<'r> molecule::prelude::Reader<'r> for ClearFilterReader<'r> {
    type Entity = ClearFilter;
    fn to_entity(&self) -> Self::Entity {
        ClearFilter::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ClearFilterReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 0;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=0]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> ClearFilterReader<'r> {
    pub const NAME: &'r str = "ClearFilterReader";
    pub const FIELD_COUNT: usize = 0;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
}
impl molecule::prelude::Builder for ClearFilterBuilder {
    type Entity = ClearFilter;
    fn expected_length(&self) -> usize {
        4
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = 4u32.to_le_bytes();
        writer.write_all(&len[..])?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        ClearFilter::new_unchecked(inner.into())
    }
}
impl ClearFilterBuilder {
    pub const NAME: &'static str = "ClearFilterBuilder";
}
#[derive(Clone)]
pub struct FilteredBlock(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct FilteredBlockReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for FilteredBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for FilteredBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for FilteredBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "transactions", self.transactions())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for FilteredBlockReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "header", self.header())?;
        write!(f, ", {}: {}", "transactions", self.transactions())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct FilteredBlockBuilder {
    pub(crate) header: Header,
    pub(crate) transactions: TransactionVec,
    pub(crate) proof: MerkleProof,
}
impl ::std::default::Default for FilteredBlock {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            100, 1, 0, 0, 16, 0, 0, 0, 76, 1, 0, 0, 80, 1, 0, 0, 60, 1, 0, 0, 12, 0, 0, 0, 36, 1,
            0, 0, 24, 1, 0, 0, 52, 0, 0, 0, 56, 0, 0, 0, 88, 0, 0, 0, 96, 0, 0, 0, 104, 0, 0, 0,
            136, 0, 0, 0, 168, 0, 0, 0, 200, 0, 0, 0, 232, 0, 0, 0, 8, 1, 0, 0, 12, 1, 0, 0, 20, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0,
            0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 20, 0,
            0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        FilteredBlock::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for FilteredBlock {
    type Builder = FilteredBlockBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        FilteredBlock(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        FilteredBlockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .header(self.header())
            .transactions(self.transactions())
            .proof(self.proof())
    }
}
impl FilteredBlock {
    pub const NAME: &'static str = "FilteredBlock";
    pub fn as_reader(&self) -> FilteredBlockReader<'_> {
        FilteredBlockReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> Header {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Header::new_unchecked(self.0.slice(start, end))
    }
    pub fn transactions(&self) -> TransactionVec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        TransactionVec::new_unchecked(self.0.slice(start, end))
    }
    pub fn proof(&self) -> MerkleProof {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            MerkleProof::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            MerkleProof::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for FilteredBlockReader<'r> {
    type Entity = FilteredBlock;
    fn to_entity(&self) -> Self::Entity {
        FilteredBlock::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        FilteredBlockReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        HeaderReader::verify(&slice[offsets[0]..offsets[1]])?;
        TransactionVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        MerkleProofReader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> FilteredBlockReader<'r> {
    pub const NAME: &'r str = "FilteredBlockReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn header(&self) -> HeaderReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        HeaderReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn transactions(&self) -> TransactionVecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        TransactionVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proof(&self) -> MerkleProofReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            MerkleProofReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            MerkleProofReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for FilteredBlockBuilder {
    type Entity = FilteredBlock;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header
            + self.header.as_slice().len()
            + self.transactions.as_slice().len()
            + self.proof.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.header.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.transactions.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.proof.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.header.as_slice())?;
        writer.write_all(self.transactions.as_slice())?;
        writer.write_all(self.proof.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        FilteredBlock::new_unchecked(inner.into())
    }
}
impl FilteredBlockBuilder {
    pub const NAME: &'static str = "FilteredBlockBuilder";
    pub fn header(mut self, v: Header) -> Self {
        self.header = v;
        self
    }
    pub fn transactions(mut self, v: TransactionVec) -> Self {
        self.transactions = v;
        self
    }
    pub fn proof(mut self, v: MerkleProof) -> Self {
        self.proof = v;
        self
    }
}
#[derive(Clone)]
pub struct MerkleProof(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct MerkleProofReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for MerkleProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for MerkleProofReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for MerkleProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "indices", self.indices())?;
        write!(f, ", {}: {}", "lemmas", self.lemmas())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for MerkleProofReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "indices", self.indices())?;
        write!(f, ", {}: {}", "lemmas", self.lemmas())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct MerkleProofBuilder {
    pub(crate) indices: Uint32Vec,
    pub(crate) lemmas: Byte32Vec,
}
impl ::std::default::Default for MerkleProof {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            20, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        MerkleProof::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for MerkleProof {
    type Builder = MerkleProofBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        MerkleProof(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        MerkleProofReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .indices(self.indices())
            .lemmas(self.lemmas())
    }
}
impl MerkleProof {
    pub const NAME: &'static str = "MerkleProof";
    pub fn as_reader(&self) -> MerkleProofReader<'_> {
        MerkleProofReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn indices(&self) -> Uint32Vec {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32Vec::new_unchecked(self.0.slice(start, end))
    }
    pub fn lemmas(&self) -> Byte32Vec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32Vec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32Vec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for MerkleProofReader<'r> {
    type Entity = MerkleProof;
    fn to_entity(&self) -> Self::Entity {
        MerkleProof::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        MerkleProofReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint32VecReader::verify(&slice[offsets[0]..offsets[1]])?;
        Byte32VecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> MerkleProofReader<'r> {
    pub const NAME: &'r str = "MerkleProofReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn indices(&self) -> Uint32VecReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32VecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn lemmas(&self) -> Byte32VecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            Byte32VecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            Byte32VecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for MerkleProofBuilder {
    type Entity = MerkleProof;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.indices.as_slice().len() + self.lemmas.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.indices.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.lemmas.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.indices.as_slice())?;
        writer.write_all(self.lemmas.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        MerkleProof::new_unchecked(inner.into())
    }
}
impl MerkleProofBuilder {
    pub const NAME: &'static str = "MerkleProofBuilder";
    pub fn indices(mut self, v: Uint32Vec) -> Self {
        self.indices = v;
        self
    }
    pub fn lemmas(mut self, v: Byte32Vec) -> Self {
        self.lemmas = v;
        self
    }
}
#[derive(Clone)]
pub struct InIBD(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct InIBDReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for InIBD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for InIBDReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for InIBD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 0 {
            write!(f, "..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for InIBDReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 0 {
            write!(f, "..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct InIBDBuilder {}
impl ::std::default::Default for InIBD {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        InIBD::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for InIBD {
    type Builder = InIBDBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        InIBD(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        InIBDReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
    }
}
impl InIBD {
    pub const NAME: &'static str = "InIBD";
    pub fn as_reader(&self) -> InIBDReader<'_> {
        InIBDReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 0;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
}
impl<'r> molecule::prelude::Reader<'r> for InIBDReader<'r> {
    type Entity = InIBD;
    fn to_entity(&self) -> Self::Entity {
        InIBD::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        InIBDReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 0;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=0]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Ok(())
    }
}
impl<'r> InIBDReader<'r> {
    pub const NAME: &'r str = "InIBDReader";
    pub const FIELD_COUNT: usize = 0;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
}
impl molecule::prelude::Builder for InIBDBuilder {
    type Entity = InIBD;
    fn expected_length(&self) -> usize {
        4
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = 4u32.to_le_bytes();
        writer.write_all(&len[..])?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        InIBD::new_unchecked(inner.into())
    }
}
impl InIBDBuilder {
    pub const NAME: &'static str = "InIBDBuilder";
}
#[derive(Clone)]
pub struct Time(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct TimeReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Time {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for TimeReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Time {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "timestamp", self.timestamp())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for TimeReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "timestamp", self.timestamp())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 1 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct TimeBuilder {
    pub(crate) timestamp: Uint64,
}
impl ::std::default::Default for Time {
    fn default() -> Self {
        let v: Vec<u8> = vec![16, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        Time::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Time {
    type Builder = TimeBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Time(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        TimeReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().timestamp(self.timestamp())
    }
}
impl Time {
    pub const NAME: &'static str = "Time";
    pub fn as_reader(&self) -> TimeReader<'_> {
        TimeReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn timestamp(&self) -> Uint64 {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Uint64::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Uint64::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for TimeReader<'r> {
    type Entity = Time;
    fn to_entity(&self) -> Self::Entity {
        Time::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        TimeReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 1;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=1]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Ok(())
    }
}
impl<'r> TimeReader<'r> {
    pub const NAME: &'r str = "TimeReader";
    pub const FIELD_COUNT: usize = 1;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn timestamp(&self) -> Uint64Reader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        if count == 1 {
            Uint64Reader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[0 + 1]) as usize;
            Uint64Reader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for TimeBuilder {
    type Entity = Time;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 1 * 4;
        len_header + self.timestamp.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 1 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.timestamp.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.timestamp.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Time::new_unchecked(inner.into())
    }
}
impl TimeBuilder {
    pub const NAME: &'static str = "TimeBuilder";
    pub fn timestamp(mut self, v: Uint64) -> Self {
        self.timestamp = v;
        self
    }
}
#[derive(Clone)]
pub struct RawAlert(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct RawAlertReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for RawAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for RawAlertReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for RawAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "id", self.id())?;
        write!(f, ", {}: {}", "cancel", self.cancel())?;
        write!(f, ", {}: {}", "min_version", self.min_version())?;
        write!(f, ", {}: {}", "max_version", self.max_version())?;
        write!(f, ", {}: {}", "priority", self.priority())?;
        write!(f, ", {}: {}", "notice_until", self.notice_until())?;
        write!(f, ", {}: {}", "message", self.message())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 7 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for RawAlertReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "id", self.id())?;
        write!(f, ", {}: {}", "cancel", self.cancel())?;
        write!(f, ", {}: {}", "min_version", self.min_version())?;
        write!(f, ", {}: {}", "max_version", self.max_version())?;
        write!(f, ", {}: {}", "priority", self.priority())?;
        write!(f, ", {}: {}", "notice_until", self.notice_until())?;
        write!(f, ", {}: {}", "message", self.message())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 7 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct RawAlertBuilder {
    pub(crate) id: Uint32,
    pub(crate) cancel: Uint32,
    pub(crate) min_version: BytesOpt,
    pub(crate) max_version: BytesOpt,
    pub(crate) priority: Uint32,
    pub(crate) notice_until: Uint64,
    pub(crate) message: Bytes,
}
impl ::std::default::Default for RawAlert {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            56, 0, 0, 0, 32, 0, 0, 0, 36, 0, 0, 0, 40, 0, 0, 0, 40, 0, 0, 0, 40, 0, 0, 0, 44, 0, 0,
            0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        RawAlert::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for RawAlert {
    type Builder = RawAlertBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RawAlert(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RawAlertReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .id(self.id())
            .cancel(self.cancel())
            .min_version(self.min_version())
            .max_version(self.max_version())
            .priority(self.priority())
            .notice_until(self.notice_until())
            .message(self.message())
    }
}
impl RawAlert {
    pub const NAME: &'static str = "RawAlert";
    pub fn as_reader(&self) -> RawAlertReader<'_> {
        RawAlertReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 7;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn id(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn cancel(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn min_version(&self) -> BytesOpt {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        BytesOpt::new_unchecked(self.0.slice(start, end))
    }
    pub fn max_version(&self) -> BytesOpt {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        BytesOpt::new_unchecked(self.0.slice(start, end))
    }
    pub fn priority(&self) -> Uint32 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        let end = u32::from_le(offsets[4 + 1]) as usize;
        Uint32::new_unchecked(self.0.slice(start, end))
    }
    pub fn notice_until(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[5]) as usize;
        let end = u32::from_le(offsets[5 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn message(&self) -> Bytes {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[6]) as usize;
        if count == 7 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[6 + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RawAlertReader<'r> {
    type Entity = RawAlert;
    fn to_entity(&self) -> Self::Entity {
        RawAlert::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RawAlertReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 7;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=7]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        Uint32Reader::verify(&slice[offsets[0]..offsets[1]])?;
        Uint32Reader::verify(&slice[offsets[1]..offsets[2]])?;
        BytesOptReader::verify(&slice[offsets[2]..offsets[3]])?;
        BytesOptReader::verify(&slice[offsets[3]..offsets[4]])?;
        Uint32Reader::verify(&slice[offsets[4]..offsets[5]])?;
        Uint64Reader::verify(&slice[offsets[5]..offsets[6]])?;
        BytesReader::verify(&slice[offsets[6]..offsets[7]])?;
        Ok(())
    }
}
impl<'r> RawAlertReader<'r> {
    pub const NAME: &'r str = "RawAlertReader";
    pub const FIELD_COUNT: usize = 7;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn id(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn cancel(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn min_version(&self) -> BytesOptReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        let end = u32::from_le(offsets[2 + 1]) as usize;
        BytesOptReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn max_version(&self) -> BytesOptReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[3]) as usize;
        let end = u32::from_le(offsets[3 + 1]) as usize;
        BytesOptReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn priority(&self) -> Uint32Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[4]) as usize;
        let end = u32::from_le(offsets[4 + 1]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn notice_until(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[5]) as usize;
        let end = u32::from_le(offsets[5 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn message(&self) -> BytesReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[6]) as usize;
        if count == 7 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[6 + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for RawAlertBuilder {
    type Entity = RawAlert;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 7 * 4;
        len_header
            + self.id.as_slice().len()
            + self.cancel.as_slice().len()
            + self.min_version.as_slice().len()
            + self.max_version.as_slice().len()
            + self.priority.as_slice().len()
            + self.notice_until.as_slice().len()
            + self.message.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 7 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.id.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.cancel.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.min_version.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.max_version.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.priority.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.notice_until.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.message.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.id.as_slice())?;
        writer.write_all(self.cancel.as_slice())?;
        writer.write_all(self.min_version.as_slice())?;
        writer.write_all(self.max_version.as_slice())?;
        writer.write_all(self.priority.as_slice())?;
        writer.write_all(self.notice_until.as_slice())?;
        writer.write_all(self.message.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        RawAlert::new_unchecked(inner.into())
    }
}
impl RawAlertBuilder {
    pub const NAME: &'static str = "RawAlertBuilder";
    pub fn id(mut self, v: Uint32) -> Self {
        self.id = v;
        self
    }
    pub fn cancel(mut self, v: Uint32) -> Self {
        self.cancel = v;
        self
    }
    pub fn min_version(mut self, v: BytesOpt) -> Self {
        self.min_version = v;
        self
    }
    pub fn max_version(mut self, v: BytesOpt) -> Self {
        self.max_version = v;
        self
    }
    pub fn priority(mut self, v: Uint32) -> Self {
        self.priority = v;
        self
    }
    pub fn notice_until(mut self, v: Uint64) -> Self {
        self.notice_until = v;
        self
    }
    pub fn message(mut self, v: Bytes) -> Self {
        self.message = v;
        self
    }
}
#[derive(Clone)]
pub struct Alert(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct AlertReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Alert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for AlertReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Alert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "raw", self.raw())?;
        write!(f, ", {}: {}", "signatures", self.signatures())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for AlertReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "raw", self.raw())?;
        write!(f, ", {}: {}", "signatures", self.signatures())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 2 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct AlertBuilder {
    pub(crate) raw: RawAlert,
    pub(crate) signatures: BytesVec,
}
impl ::std::default::Default for Alert {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            72, 0, 0, 0, 12, 0, 0, 0, 68, 0, 0, 0, 56, 0, 0, 0, 32, 0, 0, 0, 36, 0, 0, 0, 40, 0, 0,
            0, 40, 0, 0, 0, 40, 0, 0, 0, 44, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
        ];
        Alert::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Alert {
    type Builder = AlertBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Alert(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        AlertReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .raw(self.raw())
            .signatures(self.signatures())
    }
}
impl Alert {
    pub const NAME: &'static str = "Alert";
    pub fn as_reader(&self) -> AlertReader<'_> {
        AlertReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn raw(&self) -> RawAlert {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        RawAlert::new_unchecked(self.0.slice(start, end))
    }
    pub fn signatures(&self) -> BytesVec {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            BytesVec::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            BytesVec::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for AlertReader<'r> {
    type Entity = Alert;
    fn to_entity(&self) -> Self::Entity {
        Alert::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        AlertReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 2;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=2]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        RawAlertReader::verify(&slice[offsets[0]..offsets[1]])?;
        BytesVecReader::verify(&slice[offsets[1]..offsets[2]])?;
        Ok(())
    }
}
impl<'r> AlertReader<'r> {
    pub const NAME: &'r str = "AlertReader";
    pub const FIELD_COUNT: usize = 2;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn raw(&self) -> RawAlertReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        RawAlertReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn signatures(&self) -> BytesVecReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        if count == 2 {
            BytesVecReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[1 + 1]) as usize;
            BytesVecReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for AlertBuilder {
    type Entity = Alert;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 2 * 4;
        len_header + self.raw.as_slice().len() + self.signatures.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 2 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.raw.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.signatures.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.raw.as_slice())?;
        writer.write_all(self.signatures.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Alert::new_unchecked(inner.into())
    }
}
impl AlertBuilder {
    pub const NAME: &'static str = "AlertBuilder";
    pub fn raw(mut self, v: RawAlert) -> Self {
        self.raw = v;
        self
    }
    pub fn signatures(mut self, v: BytesVec) -> Self {
        self.signatures = v;
        self
    }
}
#[derive(Clone)]
pub struct Identify(molecule::bytes::Bytes);
#[derive(Clone, Copy)]
pub struct IdentifyReader<'r>(&'r [u8]);
impl ::std::fmt::Debug for Identify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl<'r> ::std::fmt::Debug for IdentifyReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}(0x{})",
            Self::NAME,
            hex_string(self.as_slice()).unwrap()
        )
    }
}
impl ::std::fmt::Display for Identify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "name", self.name())?;
        write!(f, ", {}: {}", "flag", self.flag())?;
        write!(f, ", {}: {}", "client_version", self.client_version())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
impl<'r> ::std::fmt::Display for IdentifyReader<'r> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "name", self.name())?;
        write!(f, ", {}: {}", "flag", self.flag())?;
        write!(f, ", {}: {}", "client_version", self.client_version())?;
        let (_, count, _) = Self::field_offsets(&self);
        if count != 3 {
            write!(f, ", ..")?;
        }
        write!(f, " }}")
    }
}
#[derive(Debug, Default)]
pub struct IdentifyBuilder {
    pub(crate) name: Bytes,
    pub(crate) flag: Uint64,
    pub(crate) client_version: Bytes,
}
impl ::std::default::Default for Identify {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            32, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        Identify::new_unchecked(v.into())
    }
}
impl molecule::prelude::Entity for Identify {
    type Builder = IdentifyBuilder;
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Identify(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        IdentifyReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::std::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .name(self.name())
            .flag(self.flag())
            .client_version(self.client_version())
    }
}
impl Identify {
    pub const NAME: &'static str = "Identify";
    pub fn as_reader(&self) -> IdentifyReader<'_> {
        IdentifyReader::new_unchecked(self.as_slice())
    }
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn name(&self) -> Bytes {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        Bytes::new_unchecked(self.0.slice(start, end))
    }
    pub fn flag(&self) -> Uint64 {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint64::new_unchecked(self.0.slice(start, end))
    }
    pub fn client_version(&self) -> Bytes {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            Bytes::new_unchecked(self.0.slice_from(start))
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            Bytes::new_unchecked(self.0.slice(start, end))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for IdentifyReader<'r> {
    type Entity = Identify;
    fn to_entity(&self) -> Self::Entity {
        Identify::new_unchecked(self.as_slice().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        IdentifyReader(slice)
    }
    fn as_slice(&self) -> &[u8] {
        self.0
    }
    fn verify(slice: &[u8]) -> molecule::error::VerificationResult<()> {
        use molecule::error::VerificationError;
        let len = slice.len();
        if len < 4 {
            let err = VerificationError::HeaderIsBroken(Self::NAME.to_owned(), 4, len);
            Err(err)?;
        }
        let ptr: &[u32] = unsafe { ::std::mem::transmute(slice) };
        let total_size = u32::from_le(ptr[0]) as usize;
        if total_size != len {
            let err = VerificationError::TotalSizeNotMatch(Self::NAME.to_owned(), total_size, len);
            Err(err)?;
        }
        let expected = 4 + 4 * 3;
        if total_size < expected {
            let err =
                VerificationError::HeaderIsBroken(Self::NAME.to_owned(), expected, total_size);
            Err(err)?;
        }
        let mut offsets: Vec<usize> = ptr[1..=3]
            .iter()
            .map(|x| u32::from_le(*x) as usize)
            .collect();
        if offsets[0] != expected {
            let err =
                VerificationError::FirstOffsetIsShort(Self::NAME.to_owned(), expected, offsets[0]);
            Err(err)?;
        }
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            let err = VerificationError::OffsetsNotMatch(Self::NAME.to_owned());
            Err(err)?;
        }
        BytesReader::verify(&slice[offsets[0]..offsets[1]])?;
        Uint64Reader::verify(&slice[offsets[1]..offsets[2]])?;
        BytesReader::verify(&slice[offsets[2]..offsets[3]])?;
        Ok(())
    }
}
impl<'r> IdentifyReader<'r> {
    pub const NAME: &'r str = "IdentifyReader";
    pub const FIELD_COUNT: usize = 3;
    pub fn field_offsets(&self) -> (usize, usize, &[u32]) {
        let ptr: &[u32] = unsafe { ::std::mem::transmute(self.as_slice()) };
        let bytes_len = u32::from_le(ptr[0]) as usize;
        let first = u32::from_le(ptr[1]) as usize;
        let count = (first - 4) / 4;
        (bytes_len, count, &ptr[1..])
    }
    pub fn name(&self) -> BytesReader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[0]) as usize;
        let end = u32::from_le(offsets[0 + 1]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn flag(&self) -> Uint64Reader<'_> {
        let (_, _, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[1]) as usize;
        let end = u32::from_le(offsets[1 + 1]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn client_version(&self) -> BytesReader<'_> {
        let (_, count, offsets) = Self::field_offsets(self);
        let start = u32::from_le(offsets[2]) as usize;
        if count == 3 {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end = u32::from_le(offsets[2 + 1]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl molecule::prelude::Builder for IdentifyBuilder {
    type Entity = Identify;
    fn expected_length(&self) -> usize {
        let len_header = 4 + 3 * 4;
        len_header
            + self.name.as_slice().len()
            + self.flag.as_slice().len()
            + self.client_version.as_slice().len()
    }
    fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
        let len = (self.expected_length() as u32).to_le_bytes();
        writer.write_all(&len[..])?;
        let mut offset = 4 + 3 * 4;
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.name.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.flag.as_slice().len();
        }
        {
            let tmp = (offset as u32).to_le_bytes();
            writer.write_all(&tmp[..])?;
            offset += self.client_version.as_slice().len();
        }
        let _ = offset;
        writer.write_all(self.name.as_slice())?;
        writer.write_all(self.flag.as_slice())?;
        writer.write_all(self.client_version.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner).expect("write vector should be ok");
        Identify::new_unchecked(inner.into())
    }
}
impl IdentifyBuilder {
    pub const NAME: &'static str = "IdentifyBuilder";
    pub fn name(mut self, v: Bytes) -> Self {
        self.name = v;
        self
    }
    pub fn flag(mut self, v: Uint64) -> Self {
        self.flag = v;
        self
    }
    pub fn client_version(mut self, v: Bytes) -> Self {
        self.client_version = v;
        self
    }
}
