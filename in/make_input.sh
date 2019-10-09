END=20
for ((i=1;i<=END;i++)); do
    head -c $i </dev/urandom > $i.inp.bin
    head /dev/urandom | tr -dc A-Za-z0-9 | head -c $i > $i.inp.ascii
done
