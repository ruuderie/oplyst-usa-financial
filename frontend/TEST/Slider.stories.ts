import type { Meta, StoryObj } from '@storybook/vue3';

import Slider from '../components/ui/slider/Slider.vue';

const meta = {
  title: 'Slider',
  component: Slider,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Slider>;

export default meta;
type Story = StoryObj<typeof Slider>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};