ENV=flox-env-$1-$(date +%s)
DIR=/tmp/$ENV

rm -rf $DIR && mkdir -p $DIR
git checkout-index -a --prefix=$DIR/

cd $DIR

jq --arg name "$ENV" '.name = $name' .flox/env.json | sponge .flox/env.json
flox activate -- sh -u <$2

if [[ -z "$CI" ]]; then
    rm -rf $DIR
fi
