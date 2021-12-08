# `macetada/actix`

Repositório para macetar o Actix com requests.

## Subindo o server

```bash
docker build -t actix .
docker run --rm -p 8083:80 actix
```

ou:

```bash
docker pull ghcr.io/macetada/actix:main
docker run --rm -p 8083:80 ghcr.io/macetada/actix:main
```

> Este procedimento deve ser feito numa máquina servidora ou num cluster.

## Pendências

- [ ] Acesso a I/O
- [ ] Acesso a banco de dados
