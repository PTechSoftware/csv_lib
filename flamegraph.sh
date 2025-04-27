#!/bin/bash
set -e

# =========================
# Variables
# =========================
PACKAGE_NAME="csv_lib"
TEST_FUNCTION="csv::csv_reader::tests::test_file_raw"

# =========================
# Verificar permisos root
# =========================
if [ "$EUID" -ne 0 ]; then
  echo "🔒 Necesito permisos de administrador para usar DTrace."
  echo "📋 Pidiendo sudo para continuar..."
  exec sudo "$0" "$@"
fi

# =========================
# Detectar sistema operativo
# =========================
OS="$(uname -s)"
if [[ "$OS" != "Darwin" ]]; then
    echo "❌ Este script es solo para MacOS."
    exit 1
fi

# =========================
# Compilar el test
# =========================
echo "🔧 Compilando en release..."
cargo test --no-run --release --package "$PACKAGE_NAME" --lib "$TEST_FUNCTION" -- --exact

# =========================
# Detectar binario
# =========================
BIN_PATH=$(find target/release/deps -maxdepth 1 -type f -perm +111 -name "${PACKAGE_NAME}-*" | head -n 1)

if [ -z "$BIN_PATH" ]; then
  echo "❌ No se encontró el binario."
  exit 1
fi

echo "🧩 Binario de test: $BIN_PATH"

# =========================
# Ejecutar el test capturando con dtrace
# =========================
echo "🎯 Corriendo DTrace y ejecutando el test..."
sudo dtrace -n '
    profile-997 /pid == $target/ {
        @[ustack()] = count();
    }
' -c "$BIN_PATH --exact $TEST_FUNCTION" > out.stacks

# =========================
# Generar flamegraph
# =========================
echo "📈 Generando flamegraph.svg..."
stackcollapse.pl out.stacks | flamegraph.pl > flamegraph.svg

# =========================
# Abrir flamegraph
# =========================
if [ -f flamegraph.svg ]; then
    echo "🌎 Abriendo flamegraph..."
    open flamegraph.svg
else
    echo "❌ No se encontró flamegraph.svg."
fi
