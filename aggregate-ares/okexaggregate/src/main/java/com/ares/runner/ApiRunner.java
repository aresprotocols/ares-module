package com.ares.runner;
import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.Locale;
import java.util.TimeZone;

import org.apache.commons.compress.utils.Lists;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;
import org.w3c.dom.events.Event;

import com.ares.uitl.RedisUtils;
import com.huobi.client.MarketClient;
import com.huobi.client.req.market.SubMbpRefreshUpdateRequest;
import com.huobi.constant.HuobiOptions;
import com.huobi.constant.enums.DepthLevels;
import com.okcoin.commons.okex.open.api.test.ws.swap.config.WebSocketClient;
import com.okcoin.commons.okex.open.api.test.ws.swap.config.WebSocketConfig;

import lombok.extern.slf4j.Slf4j;

@Slf4j
@Component
public class ApiRunner implements CommandLineRunner{
	
	@Value("${symbol}")
	private String symbol;
	
	@Autowired
	RedisUtils redisUtils;
	
    public static MarketClient huobimarketClient = MarketClient.create(new HuobiOptions());

    private static final WebSocketClient webSocketClient = new WebSocketClient();

    public void huobbct( String symbol) throws InterruptedException{
    	huobimarketClient.subMbpRefreshUpdate(SubMbpRefreshUpdateRequest.builder().symbols(symbol).levels(DepthLevels.LEVEL_5).build(), event -> {
           	   log.info("=========huobbct=========="+event.toString());
           	   redisUtils.setCach(symbol, "huobi", event.getAsks().get(0).getPrice(),event.getTs());
        });
    }
    
	public void okexbtc(String symbol) throws InterruptedException {
		   //添加订阅频道
	 		ArrayList<String> channel = Lists.newArrayList();
	 		channel.add("swap/ticker:BTC-USD-SWAP");
		 	 WebSocketConfig.publicConnect(webSocketClient,e->{
		 		 log.info("========okexbtc==========="+e.toString());
		 		 redisUtils.setCach(symbol, "okex", e.getDouble("last"), getW3cTimeConvertString2Date(e.getString("timestamp"), "") );
		     });
		     //调用订阅方法
		     WebSocketClient.subscribe(channel);
		 }
	
	public static long getW3cTimeConvertString2Date(String date,String timeZone) {
			SimpleDateFormat format = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss.SSS'Z'", Locale.CHINESE);
			Date parse;
			try {
				parse = format.parse(date);
				return parse.getTime();
			} catch (ParseException e) {
				// TODO Auto-generated catch block
				e.printStackTrace();
		}
			return 0;
		}
    
	@Override
	public void run(String... args) throws Exception {
		if(symbol.contains("huobi")){
			huobbct("btcusdt");
		}
		if(symbol.contains("okex")) {
			okexbtc("btcusdt");
		}
		
	}
	
	
	
	
	
}

