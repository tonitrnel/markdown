pub fn is_punctuation_or_symbol(ch: char) -> bool {
    let uc = {
        let uc = ch as u32;
        if uc < 128 {
            return ch.is_ascii_punctuation();
        }
        uc
    };
    (161..=169).contains(&uc)
        || (171..=172).contains(&uc)
        || (174..=177).contains(&uc)
        || (uc == 180)
        || (182..=184).contains(&uc)
        || (uc == 187)
        || (uc == 191)
        || (uc == 215)
        || (uc == 247)
        || (706..=709).contains(&uc)
        || (722..=735).contains(&uc)
        || (741..=747).contains(&uc)
        || (uc == 749)
        || (751..=767).contains(&uc)
        || (uc == 885)
        || (uc == 894)
        || (900..=901).contains(&uc)
        || (uc == 903)
        || (uc == 1014)
        || (uc == 1154)
        || (1370..=1375).contains(&uc)
        || (1417..=1418).contains(&uc)
        || (1421..=1423).contains(&uc)
        || (uc == 1470)
        || (uc == 1472)
        || (uc == 1475)
        || (uc == 1478)
        || (1523..=1524).contains(&uc)
        || (1542..=1551).contains(&uc)
        || (uc == 1563)
        || (1565..=1567).contains(&uc)
        || (1642..=1645).contains(&uc)
        || (uc == 1748)
        || (uc == 1758)
        || (uc == 1769)
        || (1789..=1790).contains(&uc)
        || (1792..=1805).contains(&uc)
        || (2038..=2041).contains(&uc)
        || (2046..=2047).contains(&uc)
        || (2096..=2110).contains(&uc)
        || (uc == 2142)
        || (uc == 2184)
        || (2404..=2405).contains(&uc)
        || (uc == 2416)
        || (2546..=2547).contains(&uc)
        || (2554..=2555).contains(&uc)
        || (uc == 2557)
        || (uc == 2678)
        || (2800..=2801).contains(&uc)
        || (uc == 2928)
        || (3059..=3066).contains(&uc)
        || (uc == 3191)
        || (uc == 3199)
        || (uc == 3204)
        || (uc == 3407)
        || (uc == 3449)
        || (uc == 3572)
        || (uc == 3647)
        || (uc == 3663)
        || (3674..=3675).contains(&uc)
        || (3841..=3863).contains(&uc)
        || (3866..=3871).contains(&uc)
        || (uc == 3892)
        || (uc == 3894)
        || (uc == 3896)
        || (3898..=3901).contains(&uc)
        || (uc == 3973)
        || (4030..=4037).contains(&uc)
        || (4039..=4044).contains(&uc)
        || (4046..=4058).contains(&uc)
        || (4170..=4175).contains(&uc)
        || (4254..=4255).contains(&uc)
        || (uc == 4347)
        || (4960..=4968).contains(&uc)
        || (5008..=5017).contains(&uc)
        || (uc == 5120)
        || (5741..=5742).contains(&uc)
        || (5787..=5788).contains(&uc)
        || (5867..=5869).contains(&uc)
        || (5941..=5942).contains(&uc)
        || (6100..=6102).contains(&uc)
        || (6104..=6107).contains(&uc)
        || (6144..=6154).contains(&uc)
        || (uc == 6464)
        || (6468..=6469).contains(&uc)
        || (6622..=6655).contains(&uc)
        || (6686..=6687).contains(&uc)
        || (6816..=6822).contains(&uc)
        || (6824..=6829).contains(&uc)
        || (7002..=7018).contains(&uc)
        || (7028..=7038).contains(&uc)
        || (7164..=7167).contains(&uc)
        || (7227..=7231).contains(&uc)
        || (7294..=7295).contains(&uc)
        || (7360..=7367).contains(&uc)
        || (uc == 7379)
        || (uc == 8125)
        || (8127..=8129).contains(&uc)
        || (8141..=8143).contains(&uc)
        || (8157..=8159).contains(&uc)
        || (8173..=8175).contains(&uc)
        || (8189..=8190).contains(&uc)
        || (8208..=8231).contains(&uc)
        || (8240..=8286).contains(&uc)
        || (8314..=8318).contains(&uc)
        || (8330..=8334).contains(&uc)
        || (8352..=8384).contains(&uc)
        || (8448..=8449).contains(&uc)
        || (8451..=8454).contains(&uc)
        || (8456..=8457).contains(&uc)
        || (uc == 8468)
        || (8470..=8472).contains(&uc)
        || (8478..=8483).contains(&uc)
        || (uc == 8485)
        || (uc == 8487)
        || (uc == 8489)
        || (uc == 8494)
        || (8506..=8507).contains(&uc)
        || (8512..=8516).contains(&uc)
        || (8522..=8525).contains(&uc)
        || (uc == 8527)
        || (8586..=8587).contains(&uc)
        || (8592..=9254).contains(&uc)
        || (9280..=9290).contains(&uc)
        || (9372..=9449).contains(&uc)
        || (9472..=10101).contains(&uc)
        || (10132..=11123).contains(&uc)
        || (11126..=11157).contains(&uc)
        || (11159..=11263).contains(&uc)
        || (11493..=11498).contains(&uc)
        || (11513..=11516).contains(&uc)
        || (11518..=11519).contains(&uc)
        || (uc == 11632)
        || (11776..=11822).contains(&uc)
        || (11824..=11869).contains(&uc)
        || (11904..=11929).contains(&uc)
        || (11931..=12019).contains(&uc)
        || (12032..=12245).contains(&uc)
        || (12272..=12283).contains(&uc)
        || (12289..=12292).contains(&uc)
        || (12296..=12320).contains(&uc)
        || (uc == 12336)
        || (12342..=12343).contains(&uc)
        || (12349..=12351).contains(&uc)
        || (12443..=12444).contains(&uc)
        || (uc == 12448)
        || (uc == 12539)
        || (12688..=12689).contains(&uc)
        || (12694..=12703).contains(&uc)
        || (12736..=12771).contains(&uc)
        || (12800..=12830).contains(&uc)
        || (12842..=12871).contains(&uc)
        || (uc == 12880)
        || (12896..=12927).contains(&uc)
        || (12938..=12976).contains(&uc)
        || (12992..=13311).contains(&uc)
        || (19904..=19967).contains(&uc)
        || (42128..=42182).contains(&uc)
        || (42238..=42239).contains(&uc)
        || (42509..=42511).contains(&uc)
        || (uc == 42611)
        || (uc == 42622)
        || (42738..=42743).contains(&uc)
        || (42752..=42774).contains(&uc)
        || (42784..=42785).contains(&uc)
        || (42889..=42890).contains(&uc)
        || (43048..=43051).contains(&uc)
        || (43062..=43065).contains(&uc)
        || (43124..=43127).contains(&uc)
        || (43214..=43215).contains(&uc)
        || (43256..=43258).contains(&uc)
        || (uc == 43260)
        || (43310..=43311).contains(&uc)
        || (uc == 43359)
        || (43457..=43469).contains(&uc)
        || (43486..=43487).contains(&uc)
        || (43612..=43615).contains(&uc)
        || (43639..=43641).contains(&uc)
        || (43742..=43743).contains(&uc)
        || (43760..=43761).contains(&uc)
        || (uc == 43867)
        || (43882..=43883).contains(&uc)
        || (uc == 44011)
        || (uc == 64297)
        || (64434..=64450).contains(&uc)
        || (64830..=64847).contains(&uc)
        || (uc == 64975)
        || (65020..=65023).contains(&uc)
        || (65040..=65049).contains(&uc)
        || (65072..=65106).contains(&uc)
        || (65108..=65126).contains(&uc)
        || (65128..=65131).contains(&uc)
        || (65281..=65295).contains(&uc)
        || (65306..=65312).contains(&uc)
        || (65339..=65344).contains(&uc)
        || (65371..=65381).contains(&uc)
        || (65504..=65510).contains(&uc)
        || (65512..=65518).contains(&uc)
        || (65532..=65533).contains(&uc)
        || (65792..=65794).contains(&uc)
        || (65847..=65855).contains(&uc)
        || (65913..=65929).contains(&uc)
        || (65932..=65934).contains(&uc)
        || (65936..=65948).contains(&uc)
        || (uc == 65952)
        || (66000..=66044).contains(&uc)
        || (uc == 66463)
        || (uc == 66512)
        || (uc == 66927)
        || (uc == 67671)
        || (67703..=67704).contains(&uc)
        || (uc == 67871)
        || (uc == 67903)
        || (68176..=68184).contains(&uc)
        || (uc == 68223)
        || (uc == 68296)
        || (68336..=68342).contains(&uc)
        || (68409..=68415).contains(&uc)
        || (68505..=68508).contains(&uc)
        || (uc == 69293)
        || (69461..=69465).contains(&uc)
        || (69510..=69513).contains(&uc)
        || (69703..=69709).contains(&uc)
        || (69819..=69820).contains(&uc)
        || (69822..=69825).contains(&uc)
        || (69952..=69955).contains(&uc)
        || (70004..=70005).contains(&uc)
        || (70085..=70088).contains(&uc)
        || (uc == 70093)
        || (uc == 70107)
        || (70109..=70111).contains(&uc)
        || (70200..=70205).contains(&uc)
        || (uc == 70313)
        || (70731..=70735).contains(&uc)
        || (70746..=70747).contains(&uc)
        || (uc == 70749)
        || (uc == 70854)
        || (71105..=71127).contains(&uc)
        || (71233..=71235).contains(&uc)
        || (71264..=71276).contains(&uc)
        || (uc == 71353)
        || (71484..=71487).contains(&uc)
        || (uc == 71739)
        || (72004..=72006).contains(&uc)
        || (uc == 72162)
        || (72255..=72262).contains(&uc)
        || (72346..=72348).contains(&uc)
        || (72350..=72354).contains(&uc)
        || (72448..=72457).contains(&uc)
        || (72769..=72773).contains(&uc)
        || (72816..=72817).contains(&uc)
        || (73463..=73464).contains(&uc)
        || (73539..=73551).contains(&uc)
        || (73685..=73713).contains(&uc)
        || (uc == 73727)
        || (74864..=74868).contains(&uc)
        || (77809..=77810).contains(&uc)
        || (92782..=92783).contains(&uc)
        || (uc == 92917)
        || (92983..=92991).contains(&uc)
        || (92996..=92997).contains(&uc)
        || (93847..=93850).contains(&uc)
        || (uc == 94178)
        || (uc == 113820)
        || (uc == 113823)
        || (118608..=118723).contains(&uc)
        || (118784..=119029).contains(&uc)
        || (119040..=119078).contains(&uc)
        || (119081..=119140).contains(&uc)
        || (119146..=119148).contains(&uc)
        || (119171..=119172).contains(&uc)
        || (119180..=119209).contains(&uc)
        || (119214..=119274).contains(&uc)
        || (119296..=119361).contains(&uc)
        || (uc == 119365)
        || (119552..=119638).contains(&uc)
        || (uc == 120513)
        || (uc == 120539)
        || (uc == 120571)
        || (uc == 120597)
        || (uc == 120629)
        || (uc == 120655)
        || (uc == 120687)
        || (uc == 120713)
        || (uc == 120745)
        || (uc == 120771)
        || (120832..=121343).contains(&uc)
        || (121399..=121402).contains(&uc)
        || (121453..=121460).contains(&uc)
        || (121462..=121475).contains(&uc)
        || (121477..=121483).contains(&uc)
        || (uc == 123215)
        || (uc == 123647)
        || (125278..=125279).contains(&uc)
        || (uc == 126124)
        || (uc == 126128)
        || (uc == 126254)
        || (126704..=126705).contains(&uc)
        || (126976..=127019).contains(&uc)
        || (127024..=127123).contains(&uc)
        || (127136..=127150).contains(&uc)
        || (127153..=127167).contains(&uc)
        || (127169..=127183).contains(&uc)
        || (127185..=127221).contains(&uc)
        || (127245..=127405).contains(&uc)
        || (127462..=127490).contains(&uc)
        || (127504..=127547).contains(&uc)
        || (127552..=127560).contains(&uc)
        || (127568..=127569).contains(&uc)
        || (127584..=127589).contains(&uc)
        || (127744..=128727).contains(&uc)
        || (128732..=128748).contains(&uc)
        || (128752..=128764).contains(&uc)
        || (128768..=128886).contains(&uc)
        || (128891..=128985).contains(&uc)
        || (128992..=129003).contains(&uc)
        || (uc == 129008)
        || (129024..=129035).contains(&uc)
        || (129040..=129095).contains(&uc)
        || (129104..=129113).contains(&uc)
        || (129120..=129159).contains(&uc)
        || (129168..=129197).contains(&uc)
        || (129200..=129201).contains(&uc)
        || (129280..=129619).contains(&uc)
        || (129632..=129645).contains(&uc)
        || (129648..=129660).contains(&uc)
        || (129664..=129672).contains(&uc)
        || (129680..=129725).contains(&uc)
        || (129727..=129733).contains(&uc)
        || (129742..=129755).contains(&uc)
        || (129760..=129768).contains(&uc)
        || (129776..=129784).contains(&uc)
        || (129792..=129938).contains(&uc)
        || (129940..=129994).contains(&uc)
}