=begin
O ruby funciona da mesma forma que o C# pois trabalha com Garbage Collector
=end

def tentativa_de_dangling_pointer
    local = "isso é um teste"
    puts("Estou dentro do método - #{local}")
    local
end

resultado = tentativa_de_dangling_pointer()
puts(resultado)