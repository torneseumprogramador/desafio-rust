curl -X POST http://localhost:3030/alunos \
     -H "Content-Type: application/json" \
     -d '{"nome": "Novo Aluno", "matricula": "123456", "notas": "[7.0, 8.5]"}'
