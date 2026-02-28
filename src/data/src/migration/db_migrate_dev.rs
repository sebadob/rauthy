use crate::database::DB;
use crate::entity::jwk::Jwk;
use crate::entity::magic_links::{MagicLink, MagicLinkUsage};
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::deserialize;
use rauthy_error::ErrorResponse;
use std::ops::Add;
use tracing::warn;

pub async fn migrate_dev_data() -> Result<(), ErrorResponse> {
    warn!("Migrating DEV DATA - DO NOT USE IN PRODUCTION!");

    let sql = "SELECT * FROM jwks";
    let needs_migration = if is_hiqlite() {
        match DB::hql().query_as::<Jwk, _>(sql, params!()).await {
            Ok(res) => res.is_empty(),
            Err(_) => true,
        }
    } else {
        match DB::pg_query::<Jwk>(sql, &[], 8).await {
            Ok(res) => res.is_empty(),
            Err(_) => true,
        }
    };

    if !needs_migration {
        return Ok(());
    }

    // ##############################
    // for re-generating test JWKs, un-comment temporarily and comment the pre-computed JWKs below:

    // use actix_web::web;
    // use crate::entity::jwk::JwkKeyPairAlg;
    // use cryptr::{EncKeys, EncValue};
    // use jwt_simple::prelude::*;
    // use rauthy_common::utils::get_rand;
    //
    // let enc_key_active = &EncKeys::get_static().enc_key_active;
    //
    // // RSA256
    // let jwk_plain = web::block(|| {
    //     RS256KeyPair::generate(2048)
    //         .unwrap()
    //         .with_key_id(&get_rand(24))
    // })
    // .await?;
    // let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
    //     .into_bytes()
    //     .to_vec();
    // let entity = Jwk {
    //     kid: jwk_plain.key_id().as_ref().unwrap().clone(),
    //     created_at: OffsetDateTime::now_utc().unix_timestamp(),
    //     signature: JwkKeyPairAlg::RS256,
    //     enc_key_id: enc_key_active.clone(),
    //     jwk,
    // };
    // let rs256hex = hex::encode(bincode::serialize(&entity).unwrap());
    //
    // // RS384
    // let jwk_plain = web::block(|| {
    //     RS384KeyPair::generate(3072)
    //         .unwrap()
    //         .with_key_id(&get_rand(24))
    // })
    // .await?;
    // let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
    //     .into_bytes()
    //     .to_vec();
    // let entity = Jwk {
    //     kid: jwk_plain.key_id().as_ref().unwrap().clone(),
    //     created_at: OffsetDateTime::now_utc().unix_timestamp(),
    //     signature: JwkKeyPairAlg::RS384,
    //     enc_key_id: enc_key_active.clone(),
    //     jwk,
    // };
    // let rs384hex = hex::encode(bincode::serialize(&entity).unwrap());
    //
    // // RSA512
    // let jwk_plain = web::block(|| {
    //     RS512KeyPair::generate(4096)
    //         .unwrap()
    //         .with_key_id(&get_rand(24))
    // })
    // .await?;
    // let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
    //     .into_bytes()
    //     .to_vec();
    // let entity = Jwk {
    //     kid: jwk_plain.key_id().as_ref().unwrap().clone(),
    //     created_at: OffsetDateTime::now_utc().unix_timestamp(),
    //     signature: JwkKeyPairAlg::RS512,
    //     enc_key_id: enc_key_active.clone(),
    //     jwk,
    // };
    // let rs512hex = hex::encode(bincode::serialize(&entity).unwrap());
    //
    // // Ed25519
    // let jwk_plain = web::block(|| Ed25519KeyPair::generate().with_key_id(&get_rand(24))).await?;
    // let jwk = EncValue::encrypt(jwk_plain.to_der().as_slice())?
    //     .into_bytes()
    //     .to_vec();
    // let entity = Jwk {
    //     kid: jwk_plain.key_id().as_ref().unwrap().clone(),
    //     created_at: OffsetDateTime::now_utc().unix_timestamp(),
    //     signature: JwkKeyPairAlg::EdDSA,
    //     enc_key_id: enc_key_active.clone(),
    //     jwk,
    // };
    // let eddsahex = hex::encode(bincode::serialize(&entity).unwrap());
    //
    // // output all computed hex values to the terminal, so we can easily copy & paste them here
    // // to be way faster in the future
    // println!("\n\nrs256hex:\n{}\n", rs256hex);
    // println!("\n\nrs384hex:\n{}\n", rs384hex);
    // println!("\n\nrs512hex:\n{}\n", rs512hex);
    // println!("\n\neddsahex:\n{}\n", eddsahex);

    // ##############################

    // pre-computed JWKs - these should be used unless we need to re-generate new ones for testing
    let rs256hex = "18000000000000005336444d4248494c706b5752557469454a63634966455a6d76aebc650000000000000000100000000000000062564379547347616767567935797151f304000000000000010100160000625643795473476167675679357971514b9a51456ce5574ab7b8132acf4e420ca42dbf5fa8d00b4d18a44b8632f1cd975497bd52b47faee5945854d95407a1d5d9abdbb27b6e622e108a62c226933b278bd3c77b7caadd2af1789dc12a79c09a3a3f6c859c10df9ca49225eab3398b6ec7149d863a96a60e86bf9864417e33db0d9647f2d4415a350a7dc8b13824015e4a2677a3334e95b2e617a55008decb8e7d4b60343cdd1c6b8f13093f99707b889644a6a8406bb362f114963de001f2059ca7cb5321ed93e2b36ec37c4e44073f77b4255a23af224a11c0af0f8648cc3f38555e324fbbd815725ec029430ea939e218f2f0b1a0efc8cbaf7d1d314e6beeefe485cae66f8a78c351fef3d869faac8a3f4a5900e48d5c790e63b66e4b1284edba4ab17262f3872b5fbcaeb7c13fa6a7b7336e759e90df31b9dca5f221f6f8733dbda7bd6800438215e2a96b8cb9806163e3ee30de1dd72d03442c4d4871ab61571873d97f9eca159242db3f1c605d9fbbd6dc4c0930d1e1cd9d653c86061289821898aa4086dab455378afab73ccc8b27c4acfe61d5097188e32cc8bdb04f0cee89d6a9610ce94bfe9ee2a29600992c61922a138bb128542749511ddc5c258e4113b24f67dfc5c2d68e1c5cd62f16df4469aa761f0923fddef5cb899d1c7616742bd86420aabb77f775a5c7aef58379c7a2d4e72aaa46a830d950a203a096a0ea8a153fce225807117cd6e09cda26f95b0a4f13be08c0b7b91a9a52d62405d53130e291d513723d5ade82a783fd4f39afa5be8ea02a44873ed608a70117ad996a4ca2769ae2844c26f5e60fb51273a64a6faa8cb9bdbe53ba2d0af97c3a8f15c2aa9769f012e58e3486dbec149b24a1064f06d5984e332585d89a2af63508518406d400b38889e7dc5dac7d369c37f76f588ef3a4568183cb34fe040d993d0d6a20bda1c103f68e77e1c9a8e6b09ec8cc3567cb2ff73ec18b73aa072e25d5077b214d6f93dc8de1dec006844184f2160fd4f79b56103b5647c95f1087b0a0f3b264309500453f1bd566acd57b224b2b85295e93d30638a9abe4c049d4d1b3b3f7c26e8a812fe4f6c33871284611e4b46c1f520dc1223d9f0246f2a869da72fc9995cdbc4a9cf6db8bac8b5b905a33e6241c70ac4ffa7e999acaa9a74a6e33faa2096f52b2200dda66078b6547c0f7966394c8e3e974b41a469853ef43913d10d5511415236242ec592e61cff8c2cc11a3358502178fd5eba2ae8738edbcfae194b3d0cc29e8c9343a9f1afc4b54a45e600274c84cbf985dccff852f22c8e286b0278d282f2da94b46b492fcf2d4fc30c40be7519aabd9be9ed81f51451f55976385a2174faeb006e4f31f657d522b238735f1d7e413cb84511e9c4d4046fd35574f293a8a37fa39e5e4563b6f64df66650ccbdcba50cc6747729225d56389600240d8ffc4146345595d8488c00769b595931c4ac56f5d5c6c5b9260bd4ce002ec6f73261c25400075f1d5a02b11345d6e580bbab1c38136f1cb9ee7d2b2031847085b2a44b3fec1c9968ade45ff917df55cf93722bff237f03de461eb7e29bee09829c4ace72a9fc1b712ce06529a386d8d68001ed129c800d750935085f3b8ec5e3d684e4ef7718eaedf81af756e89c80e166176563a4b0cf5da589a33eb1181a26d49c9f8dfae8e94a25a882722c1c44a091f75ead4821b81ba2992edd49f8bbbdb4fc173bcd986b7b8d714434aa22429442d1282884cb6f5f4e9c6e256f410b50bc047c55309e54ab8c1";
    let rs384hex = "180000000000000056695468505054385378436c347148494e505863624f33389baebc650000000001000000100000000000000062564379547347616767567935797151340700000000000001010016000062564379547347616767567935797151ebf9f9797dbb710595a68ee7e5496c822e63d9b92be2e67f2ee947ea242c5b51d679bd8e223e079230cf773f09ff127f3f8a5b4f0ddd6334a9bd27a7abb4e07f3509085180830d17b03c2c51171c18872a8ff51e06de66c854059688b625e1186bd87ddf5249f7b236c7db3f12cef94976b054a3d728ac446e414b03004e4b94f75b01980e8eb11748bc76b8c02bda2fb9614f9594f7eb0b25eb05f9bb92bc3fdbe4b7f6389f56463be95bc5ae967cef836f17c33fb79f1c57e59ddc2d516bedd24b40a3a5211bb8a8d62a4c914c3e6502505269279b8563a1efaee1e0d1614551339d3fce9652311bd65548fb5c581a83d38bb57b23eb2fd9fea70765598c7ca42f1f8353a609b0c64f9dd51a7ea4680f6e85312f901f84b44ebf04a961607ffb0d386ed0907fd386db1c7ee9dacef31cb22422be7049004848d88564e6fe8962aad3827b695aae19450cc8d43944a78009b11227828e61903a2b713e577359009989e1a2021dfefff345e0e072c4a2a1fbd2ad2d601d7e3a1952a55c828a81b5b40bf72a54f3190d548812f09ffbb685e82519492f7103a150f73c6151440d861f967a65d5969eb8fc1aef15a405841b119bf49355f4e44ee83bb2d1d6417f3ed006fb5af85c4bebe334ef43b70462a401079dc109376784db9f27e7d3f382cef57b3509a802982e7635564b328be99a16819018a07dc6834ed33d6f625c24af64abdd993c35f1fc01320e74b003f1d7932b0d7f50198392d28aea097cca1e36ff29851a00d3842c8cdd8f16a6eeb40434374363edcf16bc2719d835a8573d8366e6d25bb5cdc7615c700df6910173cc8f92db46fdd99ffae34b1daf271e895d7e726636df9af8ff3816bafc2740c14ba1dc11443025436c331cafef0c0ef0e85348d86efe7fc7eef470062422af923ef57ed94aac33805614d6ba42f9746f10f0de2e10b7f28252937f0f28f2bbc4a25c3034fd276e2dee6d63e2d142262c06ce2817355f8658d63829feb86771ad160b702a5df0205018f4443a9f46d8408de0bc73aad6e27c81262c6fce0a7761834dbba6b24aa0789ced6dd9a4b4e592c0546f1ceef9873dbae04967e4b85c168f74398901a9b25531f15f9f9f84aec78c30cc98b5ecfb001be60ccc6ab9bc2c736771644cd4c8b7cbff4af7b9899823eb4f3e84f16bebbd7e71169064847ff80cc9e49e35e0a21adb84bd534a6c76270cb77f8f2e78b6a7c18cc2c6c5aed9211759b714a63697cf2c4d77ac10142308ce035bf1d3b654efda9b1764ed32e32ded0358153879eed38de5ae03995dcaa9460b7bf6330869e6a678e8a08e3da165289fe022d8b66d8cb4b61e166cd122f4f4ee17554642a982dfa83a68cdd68dab47edd8378103f37a1cb218f00d632078f9fdbeedd7a67236bf98f49647cce4d79a68223e4afede73d4c869a4e238345a0294357adc990f46aa276452df511b7cac6738dfb87b4604a74be9339db828d8dde73c48dcfcace8e0fc09e56d1cf0addc4647ad526a48d458343c2bad50f3f942c583a5ba3a22ae92be80c9045d402c29416a9df059301ee4f16834cf19d970c9197057bf53d6c8c10560fa3e8eda476c9eaa8564707470f1eb39847ee24a76592a0f83fd2933fb085599785eb1274580a966cad2ed86018298c17920f1f4d3b75b3427b265fb98a44e75104defe7c6a45ed9e6af536ae7e02fb5cba9e45516ba7f6465ece534a2064beec30434bfdc743aa3b9d37139be2858c8161501527dbc892df7258ef04634b191d65ca93ad9d24f3e586ed8cd597317c4f4a4bb99796133cbf7a92797e57b168fb405b023314802bc7f10514d1d9a9f3e0ace9fd56dc173697156a1f118f3a141987e0237fa30a26ec52003774f4d0bdb74fffc394e84d99d6b96d04f42bd3f91735b38ea591271594330ba30d574e08d8f6ceec472418a079db5016a2199fe76b7482b672a4aeb6afeaf87034763911c507537aaacea6da8881926340981e5ff30c8e48b2dea9ce7407066ff67461a3f2680d666cb95531055137eb1a79b5844e91318dccb7abc86b3262ff861a65e44b0525c00228c81d5aeaf717457bd5d37db1e5cd96f199c77f2c2c474b49c7adc600aa42a8c9919496966a5f2c0b082a51ca8829aaaedcc90acd5c382095f1aec4ce5c5d59c126b1100b40ed30e254c98338b6befd100f36075e751c34f428daa67cd1702a309d45c0e0adba7280d3b17e32f989511f2b95b6fc604e1009b67cca4a27348dbf52aa0035192e30712eca2073b74f1fab38f298b29248912581e7c3c099f477e5b8c02e0c798ceccf944969dc1cc3e5b2f10e59ec328384b0aacc6af9f8b0fcba09741b824aa6fe52b37868c6c8d19a6152197a23ecdd19c8b07af30db59d4a77f29f08f4bb2a9435c616fcddc9c3afa6e9fe75cb62a8743bf34bcd2ad99b065b5a5741e9fbb5febab5ca1a50000b392e11590a48e365ac22e79402173a7a117665eb58e1b297fd6a0fb912cf8e813b2f96ca31a4c64ab89b7a4cead0c43cb0a4724cf7b6aa4912bf6c54d24bd5e5e99f9e51602cfa1159eb5a0cef15862";
    let rs512hex = "180000000000000078784c4d304b5a65646f613054576d6e466e724962326444baaebc650000000002000000100000000000000062564379547347616767567935797151780900000000000001010016000062564379547347616767567935797151869c1b0036db17e55cf66e2dc7a341fd2e987058d81159c3d6420ae8ce8af796ebcd633d8e1c44d656654c19e7689dd51471f691b39b3f19724e6f66115311c578693a117a5c74c96be7c20b818f4d1f1a8bd3def19434b7efa389b20b09a184f7a045ce1e2362c4a10af9843edb27186e421a7fd215c9069aa1502643d7d465ca13196943141c78855a5dcd4d9a0627ed39c956c8d494563795fb161ed5787ea2c45d5d002654ca0687ecbee720910a327282b4df79b5eee22bb89dcc268c696ba86513cfa6d0c653a394075958ef99c98d5aa6c2335213bc4dbaf1237377b00151e693d7f95247aa554ef350bb5eaabc786fb256ac69e0bdc2615871ebf8428d9ca1a1b54ac9ef24529246a416db70b5c6eb8e737aea700ae1fa597e4d1f327fdf2ba8aa3a805068fb87348a2470a511f88c4561dfd06ae3d487e3663bc80082146482c4eb1f81a5abcc2970735adf7fd6b375c61db13e7f8e4588049a0f35b5195f9da223c8bf0957c679b6c54f8e873c9193762f301c68851ef926c58e6e95a394d0d9d0a3ec28116f88831dbc20d33f4f589d303122d669b7a3f6a9923980d3f3b9bdba89ced9fb725fe6f45c8739466a02964887f900e38341d61cdd3cace6aa47671f2813969389ca3e4694cfb23ae20817cce88e35abd2c7540b3ae44eed2ca9cb2b045b61a3ad8f244be736c731a4742c0c4af3ad4306f9bbba8e0108e55b80d62dd2fd994bf400c99729d2c2e34d7c94c9b19de1fc9dcac1f8ed3b56442ae4b53cf4df2e1efbacbf9296dc3ad4f78f1236a57aaac034358123b6ef3db0b356552c53574335a5b342c60a9af1ec127a2c309d21345d884c28794f36f71dbe9fc1b5074c4740d771ed16621c98c7255bd65a7fd5eef92f0997af43336f83c41cbfbe6df82161a666b484e63dad65ae671d84b584e754c98417732f58ec603a997d043e77eb7fdaa0d4975515e67f30b8db6c44ac0fc0b7a6ef78888b3d45032c4b19ff29dad497607cb32cf248e508adec5b959fcc61cae1f4cbfe28272dbe0742ab6f191de9856427650e7b5d5763ec99489bd62480624fa24f706ac2fe96b42ae909c38aca79354083acdb489252b7c638956f4f9f89cb0dcf9ca34a8e48664cef490a2af3a3c76235cc362cc1eae7d31711731306f62ad399efd7cac5641d2df731943430fbfc79dfd60ccae55e662c17aae0e08a75594e6c01761108a22b3aebfb16f3ef17f6b6fe495605b8afe3782e3b8645d8a61fb3457f25b741722316fae59a6fed0895d7100b2c656c0adcc23322c61de33cd09a7dfbc370a4dc45c1aa6bc43bd0d6e315f833605fb2d6a6d2ce9e34f8ae499efc04ab91cffb105bbb7dc184aaea05db1f4359e9240d0d3f8f62d597331a4df24cf48c0de2842e96ce8985ccb703664c3d6dd9555fa4ce7cac6cb6fd0b73b0d89ec40c3b822fb2b935cab3a4980b7061ff2e5068ff20dc037ad06cf4c04b993c9a23f6de286b5e672848007b6ee3c3b624fb768eedffa3c46d744b4f0c98e9752e6d735e851ad1ddd298a3b25983f811088a6b14268045205aa4703b5276b627c70a069293ee33545f8ab7ea6c58c7809b698cb7856c6524f9cb8d480af35a7d8eeed8bdbf2e103e2010a84d0c64104dbabb646965993c13ffe9be433fc263cf3b663cada3c6144914abd34812ad25fe1106191f0cca54516a98c0d6c622ba779759181011fa52b19702c7f87186b5fff5b3b8693853dbb537d5711cc37d0a40487b52011a849296619cf6d9d06d1c4494c9ec2bc5c9c527cdc859deb1a37b371617480798695c61f338dc8365788e85cd45faebdfda51d9e03d567e142bb96c4f018ca2363a2502a863bf99d4f1cc8fc50dc3970f5d218a27a6b29c4639cc276ee2f0e1b3f13f139ea234d4de2511cc5a6f9aa674c2a51b4f376d93cb49bc68311102f29e6d8fad7167cd0956854919977b450c57f1481f6458923148a24e4f81500ca85ae693d8ff55b477e52b0875b6e18919d5ee888fa6fb39e4ef04ea19f5edfd0a79de0cff8e6beebaff0e57c0f8e2769073366ced1d7993f54aae7944f29c4a510e5a4bd3d61185ea6d7ef884ff5924c82cc1f0089d6a43af3c5bf81e28a7bedffd9e417abb05c8c053da3ee5d189c20e6f5f9f15d043cc4348c27541eb9d8c90167f2306868b312f32d6fe8d2a3e22cba353e3b8515e0b3d7c172fbc5f00e679ebecf314d72d4336d91ee109330c9187a1ad613f6fc6b4679c089e5ca53c080d4c5b6e10a582b025ca53da66db216048e9cd2129385fb0c2814742df56e64740a29b2434a66cf5ff34faf5d928ffe07cb6b7523b6bf6f52fdce797b11954bcec87373044eaf2b5c689c24794c2e0af1e3fa3c3a08432b920f368f3bfb3421b101500ccf4aed74fbbded5927b7283d0af1adb07afd15b775a7f1bd0a17c72daf7875b3558256ee109179586a5b85f95a77e4c171d0735f7cb9e641906bd1f547936129ba1e5648b5ffe05f89f2c1b0beb58d90d67c3304bbc3c8b59bf2a5e921e93aaead4280097dce16ddebc61db1143aafdbfc6dd237c27c46e7f71705e458ac9c2fb05c883bd71858d9b79396e0dda689f2bb16e517f1f873345326c814e81c54c90ab331edc3649ba08b4a4ccca5c4dba5ecbb982807895e7b26d668ed83087bceb715857c92d8df7762470b480fc63e70b1d161e74626ebe90a7a39f20d3ee383d2d49ee3ebbe5fa1101dfa441069e173bd8f7c6fc8e783aab757ab4c62a97b2983f752dfb3d22312aadb09bce611dff9f57411c05d0d5062d199ebdc675b286ece1c218ef72ce6debdb593468b0ba9e131daaf88a4622d80c4744480686e26b733e72b31df022dd6324f090bad0e3d2640d63d31257a43e95180b30a2d089b32da50228399f2d139976d3af228e87e5f14e6f88fcd5aeb98d307dc29d95a21a3f0f5f08e4633a5863f59fcf29581dda0686c1cb75493de86249ba020b1b329cd3c5bfdb6a5850c10be0aa93bcd08eddf7fa66f3323c58078fef72c4f838e60b483b52d0730d389315b93a5328e68726a14ad5c0fb4f370177a71a823b67e14331c58059a722d6f0b336c8d2a9fa4b5ccdff7d53a0ec6cf5b544cb7b3c417f7ef7613a3fe2c0b8350c717172731af3192850bf0fb8ae607ae68ea594f33f90a741948947d3a06ec13d31a00441d075e441af4db367b215c35d0cb73ba335ce62e642d663e5ca3a3fbce644488bc67ee2820466ed64bf8a8838f525f0f7b2aaeb201615b482f5a51cebb3aa7146e54e16d96bd13f1ae6a4a7fd5555f26d6633a30b1d0d267016085088f840c6a64d7cbc505f86f4169af0d5ea0bd19ae02b16dafe3180bb1d55bab28a497e5ba8c43524017a9f2e146fb8e2544c43a0e7c63be2";
    let eddsahex = "18000000000000006778646d6a6261616d516a6c43553464327453534c454447baaebc6500000000030000001000000000000000625643795473476167675679357971516200000000000000010100160000625643795473476167675679357971517a2536806a5960d51bd373a54bd1cb9691e7166bb7840a8db81be9b6744079337f9d80dad6ad3b7ab4149b9787ca5f1aefc4da11ba357293f2792c7838b73598be76eb5d2e0deb8f7d4dda4c";

    let mut jwks = Vec::with_capacity(4);
    let entity = deserialize::<Jwk>(hex::decode(rs256hex).unwrap().as_slice())?;
    jwks.push(entity);
    let entity = deserialize::<Jwk>(hex::decode(rs384hex).unwrap().as_slice())?;
    jwks.push(entity);
    let entity = deserialize::<Jwk>(hex::decode(rs512hex).unwrap().as_slice())?;
    jwks.push(entity);
    let entity = deserialize::<Jwk>(hex::decode(eddsahex).unwrap().as_slice())?;
    jwks.push(entity);

    let sql = r#"
INSERT INTO
jwks (kid, created_at, signature, enc_key_id, jwk)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT DO NOTHING"#;

    for jwk in jwks {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        jwk.kid,
                        jwk.created_at,
                        jwk.signature.as_str().to_string(),
                        jwk.enc_key_id,
                        jwk.jwk
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &jwk.kid,
                    &jwk.created_at,
                    &jwk.signature.as_str(),
                    &jwk.enc_key_id,
                    &jwk.jwk,
                ],
            )
            .await?;
        }
    }

    let issuer = &RauthyConfig::get().issuer;
    let backchannel_logout_uri = format!("{issuer}dev/backchannel_logout");
    let ml = MagicLink {
        id: "2qqdUOcXECQeypBNTs7Pnp7A2zAwr0VzynyzJiIjNR1Ua9KA95dTewM56JaPIoyj".to_string(),
        user_id: "2PYV3STNz3MN7VnPjJVcPQap".to_string(),
        csrf_token: "8jINPnFznLF9o905QuE2n9CD4rTraQO4E4fOWPZTAkbgNHqM".to_string(),
        cookie: None,
        exp: Utc::now().add(chrono::Duration::days(1)).timestamp(),
        used: false,
        usage: MagicLinkUsage::PasswordReset(None).to_string(),
    };

    let sql_1 = "UPDATE clients SET backchannel_logout_uri = $1 WHERE id = 'init_client'";
    let sql_2 = r#"
INSERT INTO
magic_links (id, user_id, csrf_token, exp, used, usage)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT DO NOTHING"#;

    if is_hiqlite() {
        // make sure that the newer backchannel logout uri is set for integration tests
        DB::hql()
            .execute(sql_1, params!(backchannel_logout_uri))
            .await?;

        DB::hql()
            .execute(
                sql_2,
                params!(ml.id, ml.user_id, ml.csrf_token, ml.exp, false, ml.usage),
            )
            .await?;
    } else {
        DB::pg_execute(sql_1, &[&backchannel_logout_uri]).await?;
        DB::pg_execute(
            sql_2,
            &[
                &ml.id,
                &ml.user_id,
                &ml.csrf_token,
                &ml.exp,
                &false,
                &ml.usage,
            ],
        )
        .await?;
    }

    // make sure `init_client` has `profile` as default scope to make user picture integration
    // tests succeed
    let sql = "UPDATE clients SET default_scopes = 'email,openid,profile' WHERE id = 'init_client'";
    if is_hiqlite() {
        DB::hql().execute(sql, params!()).await?;
    } else {
        DB::pg_execute(sql, &[]).await?;
    }

    Ok(())
}
