curl -X POST http://localhost:8080/alunos \
     -H "Content-Type: application/json" \
     -d '{"nome": "João Silva", "matricula": "123456", "notas": "7,8,9"}'