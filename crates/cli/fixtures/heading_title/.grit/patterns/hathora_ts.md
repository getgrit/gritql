# Upgrade Hathora to Dedicated TS SDK

Migrate from the [legacy Hathora Cloud SDK](https://github.com/hathora/hathora-cloud-sdks/tree/main/typescript) to the [TypeScript SDK](https://github.com/hathora/cloud-sdk-typescript).

tags: #migration, #sdk, #typescript, #hathora, #speakeasy

```grit
engine marzano(0.1)
language js

pattern update_method_name() {
  or {
    `$x.createPrivateLobby($args)` => `$x.createPrivateLobbyDeprecated($args)`,
    `$x.createPublicLobby($args)` => `$x.createPublicLobbyDeprecated($args)`,
    `$x.createRoom($args)` => `$x.createRoomDeprecated($args)`,
    `$x.destroyRoom($args)` => `$x.destroyRoomDeprecated($args)`,
    `$x.getActiveRoomsForProcess($args)` => `$x.getActiveRoomsForProcessDeprecated($args)`,
    `$x.getConnectionInfo($args)` => `$x.getConnectionInfoDeprecated($args)`,
    `$x.getInactiveRoomsForProcess($args)` => `$x.getInactiveRoomsForProcessDeprecated($args)`,
    `$x.getRoomInfo($args)` => `$x.getRoomInfoDeprecated($args)`,
    `$x.listActivePublicLobbies($args)` => `$x.listActivePublicLobbiesDeprecated($args)`,
    `$x.suspendRoom($args)` => `$x.suspendRoomDeprecated($args)`
  }
}

pattern unwrap_method() {
  or {
    `await $x.createApp($args)` as $call => `($call).application`,
    `await $x.createBuild($args)` as $call => `($call).build`,
    `await $x.createDeployment($args)` as $call => `($call).deployment`,
    `await $x.createLobby($args)` as $call => `($call).lobby`,
    `await $x.createLocalLobby($args)` as $call => `($call).lobby`,
    `await $x.createPrivateLobby($args)` as $call => `($call).lobby`,
    `await $x.createPrivateLobbyDeprecated($args)` as $call => `($call).roomId`,
    `await $x.createPublicLobby($args)` as $call => `($call).lobby`,
    `await $x.createPublicLobbyDeprecated($args)` as $call => `($call).roomId`,
    `await $x.createRoom($args)` as $call => `($call).connectionInfoV2`,
    `await $x.createRoomDeprecated($args)` as $call => `($call).roomId`,
    `await $x.getActiveRoomsForProcess($args)` as $call => `($call).roomWithoutAllocations`,
    `await $x.getActiveRoomsForProcessDeprecated($args)` as $call => `($call).roomWithoutAllocations`,
    `await $x.getAppInfo($args)` as $call => `($call).application`,
    `await $x.getApps($args)` as $call => `($call).applicationWithDeployments`,
    `await $x.getBalance($args)` as $call => `($call).getBalance200ApplicationJSONDoubleNumber`,
    `await $x.getBuildInfo($args)` as $call => `($call).build`,
    `await $x.getBuilds($args)` as $call => `($call).builds`,
    `await $x.getConnectionInfo($args)` as $call => `($call).connectionInfoV2`,
    `await $x.getConnectionInfoDeprecated($args)` as $call => `($call).connectionInfo`,
    `await $x.getDeploymentInfo($args)` as $call => `($call).deployment`,
    `await $x.getDeployments($args)` as $call => `($call).deployments`,
    `await $x.getInactiveRoomsForProcess($args)` as $call => `($call).roomWithoutAllocations`,
    `await $x.getInactiveRoomsForProcessDeprecated($args)` as $call => `($call).roomWithoutAllocations`,
    `await $x.getInvoices($args)` as $call => `($call).invoices`,
    `await $x.getLobbyInfo($args)` as $call => `($call).lobby`,
    `await $x.getLogsForApp($args)` as $call => `($call).getLogsForApp200TextPlainByteString`,
    `await $x.getLogsForDeployment($args)` as $call => `($call).getLogsForDeployment200TextPlainByteString`,
    `await $x.getLogsForProcess($args)` as $call => `($call).getLogsForProcess200TextPlainByteString`,
    `await $x.getMetrics($args)` as $call => `($call).metricsResponse`,
    `await $x.getPaymentMethod($args)` as $call => `($call).paymentMethod`,
    `await $x.getPingServiceEndpoints($args)` as $call => `($call).discoveryResponse`,
    `await $x.getProcessInfo($args)` as $call => `($call).process`,
    `await $x.getRoomInfo($args)` as $call => `($call).room`,
    `await $x.getRoomInfoDeprecated($args)` as $call => `($call).room`,
    `await $x.getRunningProcesses($args)` as $call => `($call).processWithRooms`,
    `await $x.getStoppedProcesses($args)` as $call => `($call).processes`,
    `await $x.initStripeCustomerPortalUrl($args)` as $call => `($call).initStripeCustomerPortalUrl200ApplicationJSONString`,
    `await $x.listActivePublicLobbies($args)` as $call => `($call).lobbies`,
    `await $x.listActivePublicLobbiesDeprecated($args)` as $call => `($call).lobbies`,
    `await $x.loginAnonymous($args)` as $call => `($call).loginResponse`,
    `await $x.loginGoogle($args)` as $call => `($call).loginResponse`,
    `await $x.loginNickname($args)` as $call => `($call).loginResponse`,
    `await $x.runBuild($args)` as $call => `($call).runBuild200TextPlainByteString`,
    `await $x.sendVerificationEmail($args)` as $call => `($call).verificationEmailResponse`,
    `await $x.setLobbyState($args)` as $call => `($call).lobby`,
    `await $x.updateApp($args)` as $call => `($call).application`
  }
}

pattern rewrite_args() {
  or {
    `$x.createBuild($args)` where or {
      $args <: [$a, $b] => `$a`
    } ,
    `$x.createDeployment($args)` where or {
      $args <: [$a, $b, $c] => `$c, $a, $b`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $b, $d`
    } ,
    `$x.createLobby($args)` where or {
      $args <: [$a, $b, $c] => `$c, $a`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $d`,
      $args <: [$a, $b, $c, $d, $e] => `$c, $a, $d, $e`
    } ,
    `$x.createLocalLobby($args)` where or {
      $args <: [$a, $b, $c] => `$c, $a`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $d`,
      $args <: [$a, $b, $c, $d, $e] => `$c, $a, $d, $e`
    } ,
    `$x.createPrivateLobby($args)` where or {
      $args <: [$a, $b, $c] => `$c, $a`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $d`,
      $args <: [$a, $b, $c, $d, $e] => `$c, $a, $d, $e`
    } ,
    `$x.createPrivateLobbyDeprecated($args)` where or {
      $args <: [$a, $b] => `$a`,
      $args <: [$a, $b, $c] => `$a, $c`,
      $args <: [$a, $b, $c, $d] => `$a, $d, $c`,
      $args <: [$a, $b, $c, $d, $e] => `$a, $d, $c, $e`
    } ,
    `$x.createPublicLobby($args)` where or {
      $args <: [$a, $b, $c] => `$c, $a, $b`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $b, $d`
    } ,
    `$x.createPublicLobbyDeprecated($args)` where or {
      $args <: [$a, $b] => `$a`,
      $args <: [$a, $b, $c] => `$a, $c`,
      $args <: [$a, $b, $c, $d] => `$a, $d, $c`,
      $args <: [$a, $b, $c, $d, $e] => `$a, $d, $c, $e`
    } ,
    `$x.createRoom($args)` where or {
      $args <: [$a, $b] => `$a, $b`,
      $args <: [$a, $b, $c] => `$c, $a, $b`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $b, $d`
    } ,
    `$x.createRoomDeprecated($args)` where or {
      $args <: [$a, $b] => `$b, $a`,
      $args <: [$a, $b, $c] => `$b, $a, $c`,
      $args <: [$a, $b, $c, $d] => `$b, $a, $c, $d`
    } ,
    `$x.listActivePublicLobbiesDeprecated($args)` where or {
      $args <: [$a, $b] => `$a`,
      $args <: [$a, $b, $c] => `$a, $c`,
      $args <: [$a, $b, $c, $d] => `$a, $c, $d`,
      $args <: [$a, $b, $c, $d, $e] => `$a, $c, $d, $e`
    } ,
    `$x.loginGoogle($args)` where or {
      $args <: [$a, $b] => `$b, $a`,
      $args <: [$a, $b, $c] => `$b, $a, $c`
    } ,
    `$x.loginNickname($args)` where or {
      $args <: [$a, $b] => `$b, $a`,
      $args <: [$a, $b, $c] => `$b, $a, $c`
    } ,
    `$x.runBuild($args)` where or {
      $args <: [$a, $b, $c] => `$c, $a, $b`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $b, $d`
    } ,
    `$x.setLobbyState($args)` where or {
      $args <: [$a, $b, $c] => `$c, $a, $b`,
      $args <: [$a, $b, $c, $d] => `$c, $a, $b, $d`
    } ,
    `$x.updateApp($args)` where or {
      $args <: [$a, $b] => `$b, $a`,
      $args <: [$a, $b, $c] => `$b, $a, $c`
    }
  }
}

pattern sdk_member() {
  or {
    `AppV1Api` => `appV1`,
    `AuthV1Api` => `authV1`,
    `BillingV1Api` => `billingV1`,
    `BuildV1Api` => `buildV1`,
    `DeploymentV1Api` => `deploymentV1`,
    `DiscoveryV1Api` => `discoveryV1`,
    `LobbyV1Api` => `lobbyV1`,
    `LobbyV2Api` => `lobbyV2`,
    `LogV1Api` => `logV1`,
    `ManagementV1Api` => `managementV1`,
    `MetricsV1Api` => `metricsV1`,
    `ProcessesV1Api` => `processesV1`,
    `RoomV1Api` => `roomV1`,
    `RoomV2Api` => `roomV2`,
  }
}

function new_resource_name($old_name) {
  if ($old_name <: `AppV1Api`) {
    return `AppV1`
  } else if ($old_name <: `AuthV1Api`) {
    return `AuthV1`
  } else if ($old_name <: `BillingV1Api`) {
    return `BillingV1`
  } else if ($old_name <: `BuildV1Api`) {
    return `BuildV1`
  } else if ($old_name <: `DeploymentV1Api`) {
    return `DeploymentV1`
  } else if ($old_name <: `DiscoveryV1Api`) {
    return `DiscoveryV1`
  } else if ($old_name <: `LobbyV1Api`) {
    return `LobbyV1`
  } else if ($old_name <: `LobbyV2Api`) {
    return `LobbyV2`
  } else if ($old_name <: `LogV1Api`) {
    return `LogV1`
  } else if ($old_name <: `ManagementV1Api`) {
    return `ManagementV1`
  } else if ($old_name <: `MetricsV1Api`) {
    return `MetricsV1`
  } else if ($old_name <: `ProcessesV1Api`) {
    return `ProcessesV1`
  } else if ($old_name <: `RoomV1Api`) {
    return `RoomV1`
  } else if ($old_name <: `RoomV2Api`) {
    return `RoomV2`
  },
  return $old_name
}

any {
  $new = `"@hathora/cloud-sdk-typescript"`,
  // update constructors
  `new $class($_)` as $constructor where {
    $class <: remove_import(from=`"@hathora/hathora-cloud-sdk"`),
    $class <: sdk_member(),
    $cloud = `HathoraCloud`,
    $cloud <: ensure_import_from(source=$new),
    $constructor => `new $cloud().$class`,
  },
  bubble($old, $new) $x where {
    $x <: and {
      imported_from(from=`"@hathora/hathora-cloud-sdk"`),
      not within `new $_`,
      not within `import $_`,
      remove_import(from=`"@hathora/hathora-cloud-sdk"`)
    },
    $replacement_import = new_resource_name(old_name=$x),
    $x => $replacement_import,
    $replacement_import <: ensure_import_from(source=$new),
  },
  bubble any {
    rewrite_args(),
    unwrap_method(),
    update_method_name(),
  }
}
```

## Instantiates API Resources

```js
import { AuthV1Api } from '@hathora/hathora-cloud-sdk';
const authClient = new AuthV1Api();
```

```js
import { HathoraCloud } from '@hathora/cloud-sdk-typescript';

const authClient = new HathoraCloud().authV1;
```

## Reorders method arguments

```js
import { LobbyV2Api } from '@hathora/hathora-cloud-sdk';
const lobbyClient = new LobbyV2Api();
lobbyClient.setLobbyState('my-app', 'my-room', { some: 'data' }, { request: 'ops' });
```

```js
import { HathoraCloud } from '@hathora/cloud-sdk-typescript';

const lobbyClient = new HathoraCloud().lobbyV2;
lobbyClient.setLobbyState({ some: 'data' }, 'my-app', 'my-room', {
  request: 'ops',
});
```

## Renames types

```js
import { LobbyV2Api } from '@hathora/hathora-cloud-sdk';
const lobbyClient: LobbyV2Api = new LobbyV2Api();
```

```js
import { LobbyV2, HathoraCloud } from '@hathora/cloud-sdk-typescript';

const lobbyClient: LobbyV2 = new HathoraCloud().lobbyV2;
```

## Renames deprecated methods

```js
import { RoomV1Api, LobbyV1Api, LobbyV2Api } from "@hathora/hathora-cloud-sdk";
const roomClient = new RoomV1Api();
const lobbyV1Client = new LobbyV1Api();
const lobbyV2Client = new LobbyV2Api();

roomClient.destroyRoom(process.env.HATHORA_APP_ID!,
  roomId,
  { headers: { Authorization: `Bearer ${getDeveloperToken()}`, "Content-Type": "application/json" } }
);
roomClient.suspendRoom(appId, roomId, config);
await roomClient.suspendRoom(appId, roomId, config);
const lobby = await lobbyv2Client.createPrivateLobby(appId);
// Some functions were already deprecated
const roomId = await lobbyV1Client.createPrivateLobbyDeprecated(appId);
lobbyV2client.createPrivateLobby(appId);
lobbyV1Client.createPrivateLobbyDeprecated(appId);
```

```js
import { HathoraCloud } from "@hathora/cloud-sdk-typescript";

const roomClient = new HathoraCloud().roomV1;
const lobbyV1Client = new HathoraCloud().lobbyV1;
const lobbyV2Client = new HathoraCloud().lobbyV2;

roomClient.destroyRoomDeprecated(process.env.HATHORA_APP_ID!,
  roomId,
  { headers: { Authorization: `Bearer ${getDeveloperToken()}`, "Content-Type": "application/json" } }
);
roomClient.suspendRoomDeprecated(appId, roomId, config);
await roomClient.suspendRoomDeprecated(appId, roomId, config);
const lobby = (await lobbyv2Client.createPrivateLobbyDeprecated(appId)).lobby;
// Some functions were already deprecated
const roomId = (await lobbyV1Client.createPrivateLobbyDeprecated(appId)).roomId;
lobbyV2client.createPrivateLobbyDeprecated(appId);
lobbyV1Client.createPrivateLobbyDeprecated(appId);
```

## Unwraps responses in-place

Responses often have a new intervening wrapper key for the response data. For instance, `setLobbyState` returns data under `.lobby`.

```js
import { LobbyV2Api } from '@hathora/hathora-cloud-sdk';
const lobbyClient = new LobbyV2Api();
const { state } = await lobbyClient.setLobbyState(
  'my-app',
  'my-room',
  { some: 'data' },
  { request: 'ops' },
);
```

```js
import { HathoraCloud } from '@hathora/cloud-sdk-typescript';

const lobbyClient = new HathoraCloud().lobbyV2;
const { state } = (
  await lobbyClient.setLobbyState({ some: 'data' }, 'my-app', 'my-room', {
    request: 'ops',
  })
).lobby;
```
