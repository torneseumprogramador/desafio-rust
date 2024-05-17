if (window.__TAURI__) {
    console.log(':: Tauri iniciado ::');
} else {
    console.error('window.__TAURI__ não está definido. Este código está rodando no ambiente Tauri?');
}

async function adiciona_na_lista() {
    try {
        let nome = document.getElementById("nome").value;

        console.log(nome);
        let cliente = { 
            id: 1,
            nome: nome,
            endereco: "Rua reste"
        };

        let clientes = await window.__TAURI_INVOKE__('adiciona_na_lista', { cliente });
        console.log(clientes);
        adiciona_na_tabela(clientes);
    } catch (e) {
        console.log('Erro ao invocar função Rust:', e);
    }
}

function adiciona_na_tabela(clientes){
    // Cria uma tabela
    const tabela = document.createElement('table');
    tabela.setAttribute('border', '1');

    // Adiciona cabeçalho à tabela
    const cabecalho = tabela.createTHead();
    const linhaCabecalho = cabecalho.insertRow();
    const colunas = ['ID', 'Nome', 'Endereço'];
    colunas.forEach(texto => {
        const celula = linhaCabecalho.insertCell();
        celula.textContent = texto;
    });

    // Adiciona os dados dos clientes na tabela
    const corpoTabela = tabela.createTBody();
    clientes.forEach(cliente => {
        const linha = corpoTabela.insertRow();
        Object.values(cliente).forEach(texto => {
            const celula = linha.insertCell();
            celula.textContent = texto;
        });
    });

    // Limpa o conteúdo anterior e adiciona a nova tabela ao div
    const container = document.getElementById('tabelaClientes');
    container.innerHTML = ''; // Limpa conteúdo anterior
    container.appendChild(tabela); // Adiciona a nova tabela
}

async function retorno_do_rust() {
    if (window.__TAURI__) {
        try {
            let clientes = await window.__TAURI_INVOKE__('funcao_com_retorno');
            adiciona_na_tabela(clientes);
        } catch (e) {
            console.log('Erro ao invocar função Rust:', e);
        }
    } else {
        console.error('window.__TAURI__ não está disponível.');
    }
}

async function escreve_nome_no_rust() {
    if (window.__TAURI__) {
        try {
            const nome = document.getElementById("nome").value;
            await window.__TAURI_INVOKE__('escreve_nome_no_rust', { nome });
        } catch (e) {
            console.log('Erro ao invocar função Rust:', e);
        }
    } else {
        console.error('window.__TAURI__ não está disponível.');
    }
}
