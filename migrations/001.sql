CREATE TABLE IF NOT EXISTS alunos (
    nome VARCHAR(150) NOT NULL,
    matricula VARCHAR(10) NOT NULL,
    notas VARCHAR(255),
    PRIMARY KEY (matricula)
);
