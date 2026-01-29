use zbus::{Result, proxy};

#[proxy(
    interface = "com.github.altdesktop.playerctld",
    default_service = "org.mpris.MediaPlayer2.playerctld",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait Playerctld {
	#[zbus(signal)]
    fn active_player_change_begin(&self, name: String) -> zbus::Result<()>;

    #[zbus(signal)]
    fn active_player_change_end(&self, name: String) -> zbus::Result<()>;

	#[zbus(property)]
    fn player_names(&self) -> Result<Vec<String>>;

    fn shift(&self) -> Result<String>;

    fn unshift(&self) -> Result<String>;
}
