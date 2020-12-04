package com.ares.controller;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.alibaba.fastjson.JSONObject;
import com.ares.uitl.R;
import com.ares.uitl.RedisUtils;




@RestController
@RequestMapping("/api")
public class ApiController {
		
	@Autowired
	RedisUtils redisUtils;
	
	@RequestMapping("getprice/{symbol}")
	private R getprice(@PathVariable("symbol") String symbol) {
		return R.ok().put("data", redisUtils.getCach(symbol));
	}
	
	@RequestMapping("getprice/{symbol}/{market}")
	private R getprice(@PathVariable("symbol") String symbol,@PathVariable("market") String market) {
		return R.ok().put("data", JSONObject.parseObject(redisUtils.getCach(symbol,market)+""));
	}
	
	
	
}
