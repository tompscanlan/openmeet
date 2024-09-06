import { Client } from "cassandra-driver";

const client = new Client({
  contactPoints: ["cassandra1.int.butterhead.net"],
  localDataCenter: "datacenter1",
  keyspace: "openmeet",
});

export default client;
