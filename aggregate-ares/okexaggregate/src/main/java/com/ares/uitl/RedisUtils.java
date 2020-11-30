package com.ares.uitl;

import java.math.BigDecimal;
import java.util.Map;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Component;

import com.alibaba.fastjson.JSONObject;

@Component
public class RedisUtils {
	
	@Autowired
	RedisTemplate<String,String> redis;
	
	public void setCach(String symbol,String market,double price,Long ts){
		redis.opsForHash().put(symbol,market, JSONObject.toJSONString(new PriceModel(market, symbol, price,ts)));
	}
	
	
	public Map<Object, Object> getCach(String symbol){
			return redis.opsForHash().entries(symbol);
	}
	
	
	public Object getCach(String symbol,String market){
		return redis.opsForHash().get(symbol, market);
}
	
}
