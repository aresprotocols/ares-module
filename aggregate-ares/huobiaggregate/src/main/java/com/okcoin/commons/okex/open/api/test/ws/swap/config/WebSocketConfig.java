package com.okcoin.commons.okex.open.api.test.ws.swap.config;

import java.util.Map;
import java.util.Optional;

import com.huobi.utils.ResponseCallback;
import com.okcoin.commons.okex.open.api.bean.other.SpotOrderBook;

import net.sf.json.JSONObject;

public class WebSocketConfig {

    private static final String SERVICE_URL = "wss://real.okex.com:8443/ws/v3";

   /* 模拟盘ws域名
    private static final String SERVICE_URL = "wss://real.okex.com:8443/ws/v3?brokerId=9999";
    */

    private static final String API_KEY = "";
    private static final String SECRET_KEY = "";
    private static final String PASSPHRASE = "";



    public static void publicConnect(WebSocketClient webSocketClient,ResponseCallback<JSONObject> callback) {
        System.out.println(SERVICE_URL);
        WebSocketClient.connection(SERVICE_URL);
        WebSocketClient.callback=callback;
    }

    public static void loginConnect(WebSocketClient webSocketClient) {
        System.out.println(SERVICE_URL);
        //与服务器建立连接
        WebSocketClient.connection(SERVICE_URL);
        //登录账号,用户需提供 api-key，passphrase,secret—key 不要随意透漏 ^_^
        WebSocketClient.login(API_KEY , PASSPHRASE , SECRET_KEY);
    }

}
