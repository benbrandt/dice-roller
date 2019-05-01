const builder = require("botbuilder");
const request = require("request-promise-native");
const Sentry = require("@sentry/node");

Sentry.init({
  dsn: "https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468"
});

const connector = new builder.ChatConnector({
  appId: process.env.MICROSOFT_APP_ID,
  appPassword: process.env.MICROSOFT_APP_PASSWORD
});

//this is needed to make connector.listen() method work with AWS Lambda
module.exports.hello = lambda(connector);

// Create your bot with a function to receive messages from the user
const inMemoryStorage = new builder.MemoryBotStorage();
const bot = new builder.UniversalBot(connector, function(session) {
  request({
    uri: "https://morning-eyrie-18336.herokuapp.com/",
    qs: { dice: session.message.text },
    json: true
  })
    .then(response => {
      session.send(
        `${response.rolls.map(r => `d${r.dice}: ${r.value}`).join("\n")}\n${
          response.instruction
        }: ${response.total}`
      );
    })
    .catch(error => {
      session.send(error.response.body.message);
    });
}).set("storage", inMemoryStorage); // Register in-memory storage ;

//this is needed to make connector.listen() method work with AWS Lambda
function lambda(connector) {
  let listener = connector.listen();
  let handler = (event, context, callback) => {
    let reqWrapper = {
      body: JSON.parse(event.body),
      headers: event.headers
    };
    let statusCode;
    let resWrapper = {
      status: code => {
        statusCode = code;
      },
      end: () => {
        callback(null, { statusCode: statusCode });
      }
    };
    listener(reqWrapper, resWrapper);
  };
  return handler;
}
