use reqwest;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::result::Result;

pub trait SiteDownloader {
    fn get_radar_meta(&self) -> Result<String, Box<dyn Error>>;
    fn get_radar_image(&self, rel_path: &str) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub struct MeteoArsoGovSiDownloader;
pub struct DummySiteDownloader;

impl MeteoArsoGovSiDownloader {
    pub fn new() -> MeteoArsoGovSiDownloader {
        MeteoArsoGovSiDownloader {}
    }
}

impl DummySiteDownloader {
    pub fn new() -> DummySiteDownloader {
        DummySiteDownloader {}
    }
}

impl SiteDownloader for MeteoArsoGovSiDownloader {
    fn get_radar_meta(&self) -> Result<String, Box<dyn Error>> {
        Ok(reqwest::get(
            "http://www.meteo.si/uploads/probase/www/nowcast/inca/inca_si0zm_data.json?prod=si0zm",
        )?
        .text()?)
    }

    fn get_radar_image(&self, rel_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut resp = reqwest::get(format!("{}{}", "http://www.meteo.si", rel_path).as_str())?;
        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf)?;
        Ok(buf)
    }
}

impl SiteDownloader for DummySiteDownloader {
    fn get_radar_meta(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from("[{\"mode\":\"ANL\",\"path\":\"0.png\",\"date\":\"201911210245\",\"hhmm\":\"0245\",\"bbox\":\"44.67,12.1,47.42,17.44\",\"width\":\"800\",\"height\":\"600\",\"valid\":\"2019-11-21T02:45:00Z\"}]"))
    }

    fn get_radar_image(&self, rel_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let file = File::open("test_fixtures/inca_si0zm_20191115-1830+0000.png")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;
        Ok(contents)
    }
}
