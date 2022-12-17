<template>
	<view>
		<view class="top">
			<view @click="open(item.id)" class="software" v-for="(item,index) in taskArr" :key="index">
				<view class="">
					<text>{{item.name}}</text>
				</view>
				<text>{{item.path}}</text>
			</view>
		</view>
		<view class="img">
			<image :src="imgSrc" mode=""></image>
		</view>
		<view class="online">
			<view class="status">
				<view class="icon" :style="ip ? 'background: #00a60b;' : ''">

				</view>
				<view class="">
					<text>{{ ip ? '在线' : '离线'}} </text>
					<text>{{ip}}</text>
				</view>
			</view>
			<view @click="scan()" class="">
				重新扫描
			</view>
		</view>
		<view class="log">
			<text v-for="(item,index) in logArr" :key="index">{{item}}</text>
		</view>
	</view>
</template>

<script>
	export default {
		data() {
			return {
				taskArr: [],
				logArr: [],
				ip: '',
				isScan: false,
				imgSrc: ''
			}
		},
		onLoad() {
			this.ip = uni.getStorageSync('ip')
			if (!this.ip) {
				this.scan();
			}
			this.getTask(this.ip)
			this.getImg()
		},
		methods: {
			getImg() {
				setInterval(() => {
					if (this.ip) {
						this.imgSrc = `http://${this.ip}:8000/public/screen.png?t=${new Date().getTime()}`
						uni.request({
							url: `http://${this.ip}:8000/get_img`,
							success: (res) => {
							},
							fail: (res) => {}
						});
					}
				},5000)

			},
			async scan() {
				if (this.isScan) {
					uni.showToast({
						title: "正在扫描，请稍等...",
						position: 'bottom'
					})
					return
				}
				this.isScan = true
				this.ip = ''
				this.logArr = []
				for (var i = 0; i < 255; i++) {
					await this.scanIP(i);
				}

				this.isScan = false
			},
			async getTask(ip) {
				return new Promise((resolve, reject) => {
					uni.request({
						url: `http://${ip}:8000/get_task`,
						timeout: 100,
						success: (res) => {
							if (res.data) {
								this.taskArr = res.data;
								this.ip = ip
								resolve()
							}
						},
						fail: (res) => {
							resolve()
						}
					});
				});
			},
			async scanIP(i) {
				return new Promise(async (resolve) => {
					for (var j = 0; j < 255; j++) {
						let ip = `192.168.${i}.${j}`
						this.logArr.unshift(`正在扫描 ip address: ${ip}`);
						await this.getTask(ip)
						if (this.ip) {
							this.isScan = false
							uni.setStorageSync('ip', this.ip)
							return
						}
					}
					resolve()
				});
			},
			open(id) {
				uni.showLoading({
					title: "正在打开软件...",
					mask: true
				})

				uni.request({
					url: `http://${this.ip}:8000/add_task/c0dcf2ce03e8f725ad35fe2fefef005b?id=${id}`,
					success: (res) => {
						uni.hideLoading()
						uni.showToast({
							title: "软件已打开",
							position: 'bottom'
						})
					}
				});
			}
		}
	}
</script>

<style lang="scss">
	.top {
		height: 30vh;
		overflow-y: auto;
	}

	.img {
		height: 36vh;

		image {
			width: 100%;
		}
	}


	.log {
		width: 100%;
		height: 30vh;
		background: #000;
		color: #fff;
		font-size: 24rpx;
		display: flex;
		flex-direction: column;
		overflow-y: auto;
		padding: 5px 0;
		box-sizing: border-box;

		text {
			padding: 5rpx 10rpx;
		}
	}

	.software {
		background-color: #fff;
		height: 80rpx;
		padding: 30rpx;
		border-bottom: 2rpx solid #f8f8f8;
		overflow-y: auto;

		>view {
			display: flex;
			align-items: center;
		}

		>text:last-of-type {
			font-size: 24rpx;
			color: #787878;
		}
	}

	.online {
		height: 4vh;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 20rpx;
		font-size: 24rpx;

		.status {
			display: flex;
			align-items: center;
			box-sizing: border-box;

			.icon {
				background: #c70003;
				width: 20rpx;
				height: 20rpx;
				border-radius: 20rpx;
				margin-right: 10rpx;
			}
		}

		>view:last-of-type {
			background: linear-gradient(135deg, #6e8efb, #a777e3);
			color: #fff;
			padding: 4rpx 20rpx;
			border-radius: 4rpx;
		}
	}
</style>
