#!/bin/bash

# Script para rodar simulação com logs automáticos

# Criar diretório de logs se não existir
mkdir -p logs

# Parar simulações anteriores
pkill -f "world_sim_simple.*--headless" 2>/dev/null || true

# Gerar timestamp para o arquivo de log
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
LOG_FILE="logs/simulation-$TIMESTAMP.log"

echo "Iniciando simulação... Logs serão salvos em $LOG_FILE"

# Rodar simulação em background com logs
RUST_LOG=info cargo run --release --bin world_sim_simple -- --headless --ticks 1000 > "$LOG_FILE" 2>&1 &

SIM_PID=$!

echo "Simulação rodando com PID $SIM_PID"
echo "Arquivo de log: $LOG_FILE"
echo ""
echo "Para acompanhar os logs em tempo real:"
echo "  tail -f $LOG_FILE"
echo ""
echo "Para buscar informações específicas:"
echo "  grep -E '(SPAWN|peasant|entities|Found)' $LOG_FILE"
echo "  grep -E '\[SPAWN\]' $LOG_FILE"
echo "  grep -E 'WARNING|ERROR' $LOG_FILE"
echo ""
echo "Para parar a simulação:"
echo "  kill $SIM_PID"
echo ""

# Esperar um pouco e mostrar logs iniciais
sleep 3
if [ -f "$LOG_FILE" ]; then
    echo "=== Logs iniciais ==="
    head -20 "$LOG_FILE"
    echo ""
    echo "=== Status de spawn ==="
    grep -E "\[SPAWN\]" "$LOG_FILE" | head -10
fi