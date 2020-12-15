# Run Verfploter       <origin>         <anycast>     <hitlist>     <geo-location>       <geo-location>
#verfploeter cli start nl-ens-anycast02 145.100.118.1   ./data/iplist -a data/GeoLite2-ASN.mmdb    -c data/GeoLite2-Country.mmdb | tee data/verfploeter_all_`date +%Y-%d-%m`.csv
#verfploeter cli start 2001:67c:2564:a183:1c52:d992:f21:227b 145.100.118.1   ./data/iplist -a data/GeoLite2-ASN.mmdb    -c data/GeoLite2-Country.mmdb | tee data/verfploeter_all_`date +%Y-%d-%m`.csv
#verfploeter cli start 130.89.109.182  130.89.109.182   ./data/iplist -a data/GeoLite2-ASN.mmdb    -c data/GeoLite2-Country.mmdb > data/verfploeter_all_`date +%Y-%d-%m`.csv
#verfploeter cli start us-was-anycast01 145.100.118.1   ./data/hitlist.txt -a data/GeoLite2-ASN.mmdb -c data/GeoLite2-Country.mmdb | tee data/verfploeter_all_`date +%Y%d%m-%H%M`.csv
#verfploeter cli start br-poa-anycast01 145.100.118.1   ./data/hitlist.txt -a data/GeoLite2-ASN.mmdb -c data/GeoLite2-Country.mmdb | tee data/verfploeter_all_`date +%Y%d%m-%H%M`.csv
verfploeter cli start br-poa-anycast01 145.100.118.1   ./data/iplist -a data/GeoLite2-ASN.mmdb -c data/GeoLite2-Country.mmdb | tee data/verfploeter_all_`date +%Y%d%m-%H%M`.csv
