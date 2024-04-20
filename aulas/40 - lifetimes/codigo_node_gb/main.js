/*
O Node funciona da mesma forma que o C# pois trabalha com Garbage Collector
*/

var tentativaDeDanglingPointer = () => {
    let local = "isso é um teste";
    console.log(`Estou dentro do método - ${local}`);
    return local;
}

const resultado = tentativaDeDanglingPointer();
console.log(resultado);