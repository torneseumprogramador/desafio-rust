'''
O python funciona da mesma forma que o C# pois trabalha com Garbage Collector
'''

def tentativa_de_dangling_pointer():
    local = "isso é um teste"
    print(f"Estou dentro do método - {local}")
    return local

resultado = tentativa_de_dangling_pointer()
print(resultado)