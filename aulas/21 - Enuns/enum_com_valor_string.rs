enum EnderecoIp {
    V4(String),
    V6(String),
}

fn mostrar_ip(ip: EnderecoIp){
    match ip {
        EnderecoIp::V4(valor) => {
            println!("Tenho um IPV4 com o IP {}", valor);
        },
        EnderecoIp::V6(valor) => {
            println!("Tenho um IPV6 com o IP {}", valor);
        }
    }
}

fn main(){
    let ip_com_ipv6 = EnderecoIp::V6("::1".to_string());
    let ip_com_ipv4 = EnderecoIp::V4("192.168.0.1".to_string());

    mostrar_ip(ip_com_ipv4);
    mostrar_ip(ip_com_ipv6);
}

