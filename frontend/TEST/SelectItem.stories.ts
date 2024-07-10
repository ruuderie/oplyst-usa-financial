import type { Meta, StoryObj } from '@storybook/vue3';

import SelectItem from '../components/ui/select/SelectItem.vue';

const meta = {
  title: 'SelectItem',
  component: SelectItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectItem>;

export default meta;
type Story = StoryObj<typeof SelectItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};